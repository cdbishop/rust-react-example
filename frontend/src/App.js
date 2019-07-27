import React from 'react';
import logo from './logo.svg';
import './App.css';

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      data: {
        count: 0
      }
    };
  }

  componentDidMount() {
    this.queryBackend();
  }

  render() {
    return (
      <div>
        <p>Testing rust backend API</p>
        <p>Rust backend says {this.state.data.count}</p>
        <button onClick={this.queryBackend}>Query</button>
      </div>
    );
  }

  queryBackend = () => {
    fetch("/api")
      .then(result => result.json())
      .then(data => this.setState({data}))
  }
}

export default App;
