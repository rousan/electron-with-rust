import React from 'react';
import { Progress, Card, Row, Col, Button, Icon } from 'antd';
import prettyBytes from 'pretty-bytes';
import is from 'electron-is';
import { shell, remote } from 'electron';
import './index.css';
import { FileStatus } from '../../types';

const { app } = remote;

type FileItemProps = {
  file: { name: string; size: number };
  status: FileStatus;
};

type FileItemState = {
  fileIcon: string | null;
};

class FileItem extends React.Component<FileItemProps, FileItemState> {
  static async getFileIconUrl(name: string): Promise<string> {
    const img = await app.getFileIcon(name, {
      size: 'normal'
    });
    return img.toDataURL();
  }

  constructor(props: FileItemProps) {
    super(props);

    this.state = {
      fileIcon: null
    };
  }

  componentDidMount() {
    const { file } = this.props;

    FileItem.getFileIconUrl(file.name)
      .then(url => {
        this.setState({
          fileIcon: url
        });
      })
      .catch(err => {
        console.log(err);
      });
  }

  render() {
    const { file, status } = this.props;
    const { fileIcon } = this.state;

    let statusElem;
    switch (status.type) {
      case 'connecting': {
        statusElem = (
          <div>
            Please wait, connecting to
            <span>{`${status.ip}:${status.port}`}</span>
            server
          </div>
        );
        break;
      }
      case 'progress': {
        statusElem = (
          <Progress
            strokeColor={{
              '0%': '#108ee9',
              '100%': '#87d068'
            }}
            percent={+((status.progress / file.size) * 100).toExponential(1)}
            size="small"
          />
        );
        break;
      }
      case 'complete': {
        let label;
        if (is.macOS()) {
          label = 'Show in Finder';
        } else if (is.windows()) {
          label = 'Show in File Explorer';
        } else {
          label = 'Show File Location';
        }

        statusElem = (
          <div style={{ textAlign: 'left', marginTop: 5 }}>
            <Button
              type="link"
              style={{ paddingLeft: 0 }}
              onClick={() => {
                shell.showItemInFolder(status.filePath);
              }}
            >
              {label}
            </Button>
          </div>
        );
        break;
      }
      case 'error': {
        statusElem = <div style={{ color: '#ff4d4f', marginTop: 5, userSelect: 'text' }}>{status.msg}</div>;
        break;
      }
      default:
    }

    return (
      <div className="file-item">
        <Card size="small">
          <Row type="flex" align="middle">
            <Col span={2}>
              <div>{fileIcon !== null ? <img alt="File Icon" src={fileIcon} /> : <Icon type="file" />}</div>
            </Col>
            <Col span={22}>
              <div style={{ marginLeft: 2 }}>
                <div>
                  <Row>
                    <Col span={12}>
                      <div style={{ textAlign: 'left', fontSize: 13, marginLeft: 1 }}>{file.name}</div>
                    </Col>
                    <Col span={12}>
                      <div style={{ textAlign: 'right', marginRight: 13, fontSize: 12 }}>{prettyBytes(file.size)}</div>
                    </Col>
                  </Row>
                </div>
                <div>{statusElem}</div>
              </div>
            </Col>
          </Row>
        </Card>
      </div>
    );
  }
}

export default FileItem;
