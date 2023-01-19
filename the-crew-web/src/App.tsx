import { useEffect, useState } from 'react';
import {
  useRouteError,
  Routes,
  Route,
  Outlet,
  Link,
} from "react-router-dom"
import io, { Socket } from 'socket.io-client'
import './App.css';
import { AppState, Card, Player, Task, Trick } from './model';
import UserPage from './pages/User';
import HandPage from './pages/Hand';
import ObjectivesPage from './pages/Objectives';
import TricksPage from './pages/Tricks';
import ControlPanel from './pages/ControlPanel';
import SolverPage from './pages/Solver';

interface ServerToClientEvents {
  "appstate": (data: AppState) => void;
}

interface ClientToServerEvents {
}

const socket: Socket<ServerToClientEvents, ClientToServerEvents> = io({ transports: ["websocket"] });

function App() {
  const [data, setData] = useState<AppState>({
    handPage: { heldCards: [] },
    objectivePage: { tasks: [], nextAbsolute: 1, nextRelative: 1, haveLast: false, players: [] },
    tricksPage: { tricks: [] },
    controlPanel: { players: [], tricks: [] },
    solverPage: { solves: [] },
  });
  const [selectedPlayer, setSelectedPlayer] = useState<number | undefined>(0);


  const [isConnected, setIsConnected] = useState<boolean>(socket.connected);
  useEffect(() => {
    socket.on("connect", () => {
      setIsConnected(true);
    });

    socket.on("disconnect", () => {
      setIsConnected(false);
    });

    socket.on("appstate", (data) => {
      setData(data);
    });

    return () => {
      socket.off("connect");
      socket.off("disconnect");
    };
  }, []);

  return (
    <Routes>
      <Route path="/" element={<Root isConnected={isConnected} />} errorElement={<ErrorPage />}>
        <Route errorElement={<ErrorPage />}>
          <Route index element={<IndexPage />} />
          <Route path="user/" element={<UserPage players={data.objectivePage.players} selectedPlayer={selectedPlayer} setSelectedPlayer={setSelectedPlayer} />} />
          <Route path="hand/" element={<HandPage data={data.handPage} selectedPlayer={selectedPlayer} />} />
          <Route path="objectives/" element={<ObjectivesPage data={data.objectivePage} />} />
          <Route path="tricks/" element={<TricksPage data={data.tricksPage} />} />
          <Route path="controlpanel/" element={<ControlPanel data={data.controlPanel} />} />
          <Route path="solver/" element={<SolverPage data={data.solverPage} />} />
        </Route>
      </Route>
    </Routes>
  );
}

function Root({ isConnected }: { isConnected: boolean }) {
  return (
    <>
      <div id="sidebar">
        <Link to={`/`}><h1>🚀</h1></Link>
        <p>{isConnected ? "Connected" : "Disconnected"}</p>
        <nav>
          <ul>
            <li>
              <Link to={`/user/`}>User</Link>
            </li>
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
            <li>
              <Link to={`/solver/`}>Solver</Link>
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
