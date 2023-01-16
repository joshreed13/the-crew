import React from 'react';
import {
  useRouteError,
  Routes,
  Route,
  Outlet,
  Link,
} from "react-router-dom"
import './App.css';
import { AppState, Card, Player, Task, Trick } from './model';
import HandPage from './pages/Hand';
import ObjectivesPage from './pages/Objectives';
import TricksPage from './pages/Tricks';
import ControlPanel from './pages/ControlPanel';

const player1: Player = {
  name: "John",
  isCommander: true
};

const player2: Player = {
  name: "Sally",
  isCommander: false
};

const hand1: Card[] = [
  { suit: "B", value: 1 },
  { suit: "Y", value: 3 },
  { suit: "M", value: 6 },
  { suit: "G", value: 9 },
  { suit: "R", value: 4 }
];

const task1: Task = {
  id: "a",
  type: "absolute",
  order: 1,
  card: { suit: "G", value: 4 },
  player: {
    name: "John",
    isCommander: true
  }
};

const tricks: Trick[] = [{
  turns: [{
    player: player1,
    card: { suit: "Y", value: 3 },
    isLeader: true,
    isWinner: false,
    isNextToPlay: false,
  }, {
    player: player2,
    card: { suit: "B", value: 9 },
    isLeader: false,
    isWinner: false,
    isNextToPlay: false,
  }]
}];

const mystate: AppState = {
  handPage: {
    heldCards: hand1
  },
  objectivePage: {
    tasks: [task1]
  },
  tricksPage: {
    tricks: tricks
  },
  controlPanel: {
    players: [{
      player: player1,
      hand: hand1,
      tasks: [task1],
    }, {
      player: player2,
      hand: hand1,
      tasks: [],
    }],
    tricks: tricks
  },
}

class App extends React.Component<{}, AppState> {
  constructor(props: any) {
    super(props);
    this.state = mystate;
  }

  render() {
    return (
      <Routes>
        <Route path="/" element={<Root />} errorElement={<ErrorPage />}>
          <Route errorElement={<ErrorPage />}>
            <Route index element={<IndexPage />} />
            <Route path="hand/" element={<HandPage data={this.state.handPage} />} />
            <Route path="objectives/" element={<ObjectivesPage data={this.state.objectivePage} />} />
            <Route path="tricks/" element={<TricksPage data={this.state.tricksPage} />} />
            <Route path="controlpanel/" element={<ControlPanel data={this.state.controlPanel} />} />
          </Route>
        </Route>
      </Routes>
    );
  }
}

function Root() {
  return (
    <>
      <div id="sidebar">
        <Link to={`/`}><h1>🚀</h1></Link>
        <nav>
          <ul>
            <li>
              <Link to={`/hand/`}>Hand</Link>
            </li>
            <li>
              <Link to={`/objectives/`}>Objectives</Link>
            </li>
            <li>
              <Link to={`/tricks/`}>Tricks</Link>
            </li>
            <li>
              <Link to={`/controlpanel/`}>Control Panel</Link>
            </li>
          </ul>
        </nav>
      </div >
      <div id="detail">
        <Outlet />
      </div>
    </>
  );
}

function IndexPage() {
  return <div>🌎✨🚀✨🪐</div>
}

function ErrorPage() {
  const error: any = useRouteError();
  console.error(error);

  return (
    <div>
      <h1>Oops!</h1>
      <p>Sorry, an unexpected error has occurred.</p>
      <p>
        <i>{error.statusText || error.message}</i>
      </p>
    </div>
  );
}

export default App;
