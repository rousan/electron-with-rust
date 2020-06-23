import React from 'react';
import { Button } from 'antd';
import { Link } from 'react-router-dom';
import './index.css';
import { startTokioRuntime } from '../../native';

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
  }

  onButtonClick = () => {};

  render() {
    const { txt } = this.state;

    return (
      <div className="home">
        <Button type="primary" style={{ margin: 10 }} onClick={this.onButtonClick}>
          Fetch345
        </Button>
        <Link to="/foo">Foo</Link>
        <div>{txt}</div>
      </div>
    );
  }
}

export default Home;
