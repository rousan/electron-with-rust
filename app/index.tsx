import React from 'react';
import ReactDOM from 'react-dom';
import { ConnectedRouter } from 'connected-react-router';
import { Provider } from 'react-redux';
import './app.global.css';
import App from './containers/App';
import { configureStore, history } from './store/configureStore';

ReactDOM.render(
  <Provider store={configureStore()}>
    <ConnectedRouter history={history}>
      <App />
    </ConnectedRouter>
  </Provider>,
  document.getElementById('root')
);
