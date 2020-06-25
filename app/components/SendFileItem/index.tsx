import React from 'react';
import './index.css';
import { SendFileStatus, FileStatus } from '../../types';
import FileItem from '../FileItem';

type SendFileItemProps = {
  refId: string;
  file: { path: string; name: string; size: number };
  to: { ip: string; port: number };
  status: SendFileStatus;
};

type SendFileItemState = {};

class SendFileItem extends React.Component<SendFileItemProps, SendFileItemState> {
  render() {
    const { file, to, status } = this.props;

    let fileStatus: FileStatus | undefined;
    switch (status.type) {
      case 'connecting': {
        fileStatus = {
          type: 'connecting',
          ip: to.ip,
          port: to.port
        };
        break;
      }
      case 'progress': {
        fileStatus = {
          type: 'progress',
          progress: status.progress
        };
        break;
      }
      case 'complete': {
        fileStatus = {
          type: 'complete',
          filePath: file.path
        };
        break;
      }
      case 'error': {
        fileStatus = {
          type: 'error',
          msg: status.msg
        };
        break;
      }
      default:
    }

    return (
      <div className="send-file-item">
        <FileItem file={file} status={fileStatus as FileStatus} />
      </div>
    );
  }
}

export default SendFileItem;
