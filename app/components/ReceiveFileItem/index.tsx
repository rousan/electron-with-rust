import React from 'react';
import './index.css';
import { FileStatus, ReceiveFileStatus } from '../../types';
import FileItem from '../FileItem';

type ReceiveFileItemProps = {
  refId: string;
  file: { name: string; size: number };
  from: { ip: string; port: number };
  status: ReceiveFileStatus;
};

type ReceiveFileItemState = {};

class ReceiveFileItem extends React.Component<ReceiveFileItemProps, ReceiveFileItemState> {
  render() {
    const { file, status } = this.props;

    let fileStatus: FileStatus | undefined;
    switch (status.type) {
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
          filePath: status.outputPath
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
      <div className="receive-file-item">
        <FileItem file={file} status={fileStatus as FileStatus} />
      </div>
    );
  }
}

export default ReceiveFileItem;
