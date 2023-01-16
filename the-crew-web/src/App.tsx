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

class App extends React.Component<{}, AppState> {
  constructor(props: any) {
    super(props);
    this.state = {
      handPage: { heldCards: [] },
      objectivePage: { tasks: [] },
      tricksPage: { tricks: [] },
      controlPanel: { players: [], tricks: [] },
    };

    fetch('/api/appstate').then(async (response) => this.setState(await response.json() as AppState));
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
