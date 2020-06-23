import React from 'react';
import { Button } from 'antd';
import { Link } from 'react-router-dom';
import { bindActionCreators, Dispatch } from 'redux';
import { connect } from 'react-redux';
// import fs from 'fs';
import './index.css';
import { increment, decrement, incrementIfOdd, incrementAsync } from '../../actions/counter';
import { counterStateType } from '../../reducers/types';
// import { fetchUrl, asyncTask } from '../../native';
import { startTokioRuntime, fooTask } from '../../native';

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

  onButtonClick = () => {
    // console.log('clicked2444');
    // asyncTask();
    // setTimeout(() => {
    //   console.log('s');
    // }, 10000);
    // console.time('rust');
    // fetchUrl('file:///Users/rousan/cars2.json')
    //   .then(data => {
    //     console.timeEnd('rust');
    //     console.log(data);
    //   })
    //   .catch(err => {
    //     console.error(err);
    //   });
    //
    // console.time('js');
    // fs.readFile('/Users/rousan/Downloads/cars.json', { encoding: 'utf8' }, (_, data) => {
    //   const json = JSON.parse(data);
    //   console.timeEnd('js');
    //   console.log(json);
    // });
    // let data = fs::read_to_string("/Users/rousan/Downloads/cars.json").wrap()?;
    // let data = serde_json::from_str(&data).wrap()?;
    // startTokioRuntime();
    // fooTask();
    fooTask((content: string) => {
      console.log('Foo task done');
      this.setState({
        txt: content
      });
      console.log(JSON.parse(content));
    });
  };

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

function mapStateToProps(state: counterStateType) {
  return {
    counter: state.counter
  };
}

function mapDispatchToProps(dispatch: Dispatch) {
  return bindActionCreators(
    {
      increment,
      decrement,
      incrementIfOdd,
      incrementAsync
    },
    dispatch
  );
}

export default connect(mapStateToProps, mapDispatchToProps)(Home);
