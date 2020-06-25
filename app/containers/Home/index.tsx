import React, { ChangeEvent } from 'react';
import { Tabs, Upload, Icon, Input, Button, Divider, message, Empty } from 'antd';
import { remote } from 'electron';
import './index.css';
import { startTokioRuntime, startServer, sendFile, genRefId, getFileMeta } from '../../native';
import ReceiveFileItem from '../../components/ReceiveFileItem';
import SendFileItem from '../../components/SendFileItem';
import { ReceiveFileStatus, SendFileStatus } from '../../types';

const { TabPane } = Tabs;
const { Dragger } = Upload;
const { Group: ButtonGroup } = Button;

type TabType = 'send' | 'receive';

type HomeProps = {};

type HomeState = {
  activeTab: TabType;
  selectedSendFiles: File[];
  recipientIP: string;
  sendFiles: {
    [key: string]: {
      refId: string;
      file: { path: string; name: string; size: number };
      to: { ip: string; port: number };
      status: SendFileStatus;
    };
  };
  receiveFiles: {
    [key: string]: {
      refId: string;
      file: { name: string; size: number };
      from: { ip: string; port: number };
      status: ReceiveFileStatus;
    };
  };
};

class Home extends React.Component<HomeProps, HomeState> {
  constructor(props: HomeProps) {
    super(props);

    this.state = {
      activeTab: 'send',
      selectedSendFiles: [],
      recipientIP: '',
      sendFiles: {},
      receiveFiles: {}
    };
  }

  componentDidMount() {
    startTokioRuntime();
    startServer({
      port: 8888,
      // receiveFilesDir: remote.app.getPath('downloads'),
      receiveFilesDir: `${remote.app.getPath('desktop')}/electron-with-rust-outputs`,
      onStart: () => {
        console.log('Server Started');
      },
      onReceiveFileStart: (refId, from, file) => {
        const newFile = {
          refId,
          file,
          from,
          status: {
            type: 'progress',
            progress: 0
          } as ReceiveFileStatus
        };

        this.setState(prevState => {
          return {
            activeTab: 'receive',
            receiveFiles: { ...prevState.receiveFiles, [refId]: newFile }
          };
        });
      },
      onReceiveFileProgress: (refId, progress) => {
        this.updateReceiveFileStatus(refId, { type: 'progress', progress });
      },
      onReceiveFileComplete: (refId, outputPath) => {
        this.updateReceiveFileStatus(refId, { type: 'complete', outputPath });
      },
      onReceiveFileError: (refId, msg) => {
        this.updateReceiveFileStatus(refId, { type: 'error', msg });
      },
      onServerError: msg => {
        console.log('onServerError', msg);
      }
    });
  }

  updateSendFileStatus(refId: string, status: SendFileStatus) {
    this.setState(prevState => {
      const sendFiles = { ...prevState.sendFiles };

      if (sendFiles[refId]) {
        sendFiles[refId].status = status;
      } else if (status.type === 'error') {
        message.error(status.msg);
      }

      return {
        sendFiles
      };
    });
  }

  updateReceiveFileStatus(refId: string, status: ReceiveFileStatus) {
    this.setState(prevState => {
      const receiveFiles = { ...prevState.receiveFiles };

      if (receiveFiles[refId]) {
        receiveFiles[refId].status = status;
      } else if (status.type === 'error') {
        message.error(status.msg);
      }

      return {
        receiveFiles
      };
    });
  }

  queueSingleFileToSend(refId: string, filePath: string, ip: string, port: number) {
    const fileMeta = getFileMeta(filePath);

    const newFile = {
      refId,
      file: { path: filePath, name: fileMeta.name, size: fileMeta.size },
      to: { ip, port },
      status: { type: 'connecting' } as SendFileStatus
    };

    this.setState(
      prevState => {
        return {
          sendFiles: { ...prevState.sendFiles, [refId]: newFile }
        };
      },
      () => {
        sendFile({
          refId,
          ip,
          port,
          filePath,
          onSendFileStart: refId => {
            this.updateSendFileStatus(refId, { type: 'progress', progress: 0 });
          },
          onSendFileProgress: (refId, progress) => {
            this.updateSendFileStatus(refId, { type: 'progress', progress });
          },
          onSendFileComplete: refId => {
            this.updateSendFileStatus(refId, { type: 'complete' });
          },
          onSendFileError: (refId, msg) => {
            this.updateSendFileStatus(refId, { type: 'error', msg });
          }
        });
      }
    );
  }

  onTabChange(key: TabType) {
    this.setState({
      activeTab: key
    });
  }

  onChangeRecipientIP(evt: ChangeEvent<HTMLInputElement>) {
    this.setState({
      recipientIP: evt.target.value
    });
  }

  onClickQueueSendButton() {
    const { selectedSendFiles } = this.state;

    selectedSendFiles.forEach(file => {
      setImmediate(() => {
        const refId = genRefId();
        const filePath = file.path;
        const ip = '127.0.0.1';
        const port = 8888;
        this.queueSingleFileToSend(refId, filePath, ip, port);
      });
    });
  }

  onClickResetSendButton() {}

  render() {
    const { activeTab, selectedSendFiles, recipientIP, sendFiles, receiveFiles } = this.state;

    const draggerProps = {
      multiple: true,

      onRemove: (file: File) => {
        this.setState(prevState => {
          const selectedSendFiles = [...prevState.selectedSendFiles];

          const index = selectedSendFiles.indexOf(file);
          selectedSendFiles.splice(index, 1);

          return {
            selectedSendFiles
          };
        });
      },

      beforeUpload: (file: File) => {
        this.setState(prevState => {
          return {
            selectedSendFiles: [...prevState.selectedSendFiles, file]
          };
        });
        return false;
      },
      fileList: selectedSendFiles
    };

    return (
      <div className="home">
        <Tabs
          activeKey={activeTab}
          onChange={key => {
            this.onTabChange(key as TabType);
          }}
          animated={false}
          size="small"
        >
          <TabPane tab="Send Files" key="send">
            <div className="tab-send">
              <div className="send-file-box">
                <Dragger {...(draggerProps as unknown)} style={{ maxHeight: 400 }}>
                  <p className="ant-upload-drag-icon">
                    <Icon type="inbox" />
                  </p>
                  <p className="ant-upload-text">Click or drag file to this area to send</p>
                  <p className="ant-upload-hint">Sending Bulk files are also supported</p>
                </Dragger>
                <div style={{ marginTop: 20 }}>
                  <Input
                    className="input-recipient-ip"
                    addonBefore="Recipient IP"
                    placeholder="Enter recipient IP address"
                    allowClear
                    value={recipientIP}
                    onChange={evt => {
                      this.onChangeRecipientIP(evt);
                    }}
                    onPressEnter={() => this.onClickQueueSendButton()}
                  />
                </div>
                <div style={{ textAlign: 'right', marginTop: 20 }}>
                  <ButtonGroup>
                    <Button type="default" onClick={() => this.onClickResetSendButton()}>
                      Reset
                    </Button>
                    <Button type="primary" onClick={() => this.onClickQueueSendButton()}>
                      Queue to Send
                    </Button>
                  </ButtonGroup>
                </div>
              </div>
              {Object.keys(sendFiles).length > 0 ? (
                <div className="send-files-list-container">
                  <Divider type="horizontal" />
                  <div className="send-files-list-wrapper">
                    {Object.keys(sendFiles).map(key => {
                      const { refId, file, to, status } = sendFiles[key];
                      return <SendFileItem key={refId} refId={refId} file={file} to={to} status={status} />;
                    })}
                  </div>
                </div>
              ) : null}
            </div>
          </TabPane>
          <TabPane tab="Received Files" key="receive">
            <div className="tab-receive">
              <div>
                {Object.keys(receiveFiles).length > 0 ? (
                  Object.keys(receiveFiles).map(key => {
                    const { refId, file, from, status } = receiveFiles[key];
                    return <ReceiveFileItem key={refId} refId={refId} file={file} from={from} status={status} />;
                  })
                ) : (
                  <div style={{ marginTop: 100 }}>
                    <Empty description="No files received yet" />
                  </div>
                )}
              </div>
            </div>
          </TabPane>
        </Tabs>
      </div>
    );
  }
}

export default Home;
