import React from 'react';
import { Switch, Route } from 'react-router-dom';
import { hot } from 'react-hot-loader/root';
import './index.css';
import Home from '../Home';

function App() {
  return (
    <div className="app">
      <Switch>
        <Route path="/" exact component={Home} />
      </Switch>
    </div>
  );
}

let appComp;
if (process.env.NODE_ENV === 'development') {
  appComp = hot(App);
} else {
  appComp = App;
}

const Comp = appComp;
export default Comp;
