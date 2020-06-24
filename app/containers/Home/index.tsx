import React from 'react';
import { Button } from 'antd';
import { remote } from 'electron';
import './index.css';
import { startTokioRuntime, startServer, sendFile } from '../../native';

type HomeProps = {};

type HomeState = {
  txt: string;
};

class Home extends React.Component<HomeProps, HomeState> {
  constructor(props: HomeProps) {
    super(props);

    this.state = {
      txt: ''
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
        console.log('onReceiveFileStart', refId, from, file);
      },
      onReceiveFileProgress: (refId, progress) => {
        console.log('onReceiveFileProgress', refId, progress);
      },
      onReceiveFileComplete: (refId, outputPath) => {
        console.log('onReceiveFileComplete', refId, outputPath);
      },
      onReceiveFileError: (refId, msg) => {
        // check if DOM is created with the id, then use dom, otherwise use antd message.
        console.log('onReceiveFileError', refId, msg);
      },
      onServerError: msg => {
        console.log('onServerError', msg);
      }
    });
  }

  onButtonClick = () => {
    sendFile({
      ip: '127.0.0.1',
      port: 8888,
      filePath: '/Users/rousan/Downloads/Popcorn-Time-0.4.4.pkg', // 14,75,77,292 bytes
      onSendFileStart: (refId, file) => {
        console.log('onSendFileStart', refId, file);
      },
      onSendFileProgress: (refId, progress) => {
        console.log('onSendFileProgress', refId, progress);
      },
      onSendFileComplete: refId => {
        console.log('onSendFileComplete', refId);
      },
      onSendFileError: (refId, msg) => {
        // check if DOM is created with the id, then use dom, otherwise use antd message.
        console.log('onSendFileError', refId, msg);
      }
    });
  };

  render() {
    const { txt } = this.state;

    return (
      <div className="home">
        <Button type="primary" style={{ margin: 10 }} onClick={this.onButtonClick}>
          Fetch345
        </Button>
        <div>{txt}</div>
      </div>
    );
  }
}

export default Home;
