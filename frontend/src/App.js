import React from 'react';
import logo from './logo.svg';
import './App.css';

// function App() {
//   return (
//     <div className="App">
//       <header className="App-header">
//         <img src={logo} className="App-logo" alt="logo" />
//         <p>
//           Edit <code>src/App.js</code> and save to reload.
//         </p>
//         <a
//           className="App-link"
//           href="https://reactjs.org"
//           target="_blank"
//           rel="noopener noreferrer"
//         >
//           Learn React
//         </a>
//       </header>
//     </div>
//   );
// }

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      data: {
        count: 0
      }
    };
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
