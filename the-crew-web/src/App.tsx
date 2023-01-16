import React, { useEffect, useState } from 'react';
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
import HandPage from './pages/Hand';
import ObjectivesPage from './pages/Objectives';
import TricksPage from './pages/Tricks';
import ControlPanel from './pages/ControlPanel';

interface ServerToClientEvents {
  "pong": () => void;
}

interface ClientToServerEvents {
  "ping": () => void;
}

const socket: Socket<ServerToClientEvents, ClientToServerEvents> = io({ transports: ["websocket"] });

function App() {
  const [data, setData] = useState({
    handPage: { heldCards: [] },
    objectivePage: { tasks: [] },
    tricksPage: { tricks: [] },
    controlPanel: { players: [], tricks: [] },
  });

  const [isConnected, setIsConnected] = useState(socket.connected);
  const [lastPong, setLastPong] = useState<string | null>(null);
  useEffect(() => {
    socket.on("connect", () => {
      setIsConnected(true);
    });

    socket.on("disconnect", () => {
      setIsConnected(false);
    });

    socket.on("pong", () => {
      setLastPong(new Date().toISOString());
    });

    return () => {
      socket.off("connect");
      socket.off("disconnect");
      socket.off("pong");
    };
  }, []);

  const sendPing = () => {
    socket.emit("ping");
  };

  return (
    <div>
      <p>{isConnected ? "Connected" : "Disconnected"}</p>
      <p>{lastPong}</p>
      <button onClick={sendPing}>Send ping</button>
    </div>
  );
  /*
  return (
    <Routes>
      <Route path="/" element={<Root />} errorElement={<ErrorPage />}>
        <Route errorElement={<ErrorPage />}>
          <Route index element={<IndexPage />} />
          <Route path="hand/" element={<HandPage data={data.handPage} />} />
          <Route path="objectives/" element={<ObjectivesPage data={data.objectivePage} />} />
          <Route path="tricks/" element={<TricksPage data={data.tricksPage} />} />
          <Route path="controlpanel/" element={<ControlPanel data={data.controlPanel} />} />
        </Route>
      </Route>
    </Routes>
  );
  */
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
