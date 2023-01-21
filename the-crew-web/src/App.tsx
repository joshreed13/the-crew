import { useEffect, useState } from 'react';
import {
  useRouteError,
  Routes,
  Route,
  Outlet,
  Link,
} from "react-router-dom"
import io, { Socket } from 'socket.io-client'
import { AppState } from './model';
import UserPage from './pages/User';
import HandPage from './pages/Hand';
import ObjectivesPage from './pages/Objectives';
import TricksPage from './pages/Tricks';
import ControlPanel from './pages/ControlPanel';
import SolverPage from './pages/Solver';

import Container from 'react-bootstrap/Container';
import Nav from 'react-bootstrap/Nav';
import Navbar from 'react-bootstrap/Navbar'

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
    tricksPage: { tricks: [], heldCards: [] },
    controlPanel: { players: [] },
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
          <Route path="tricks/" element={<TricksPage data={data.tricksPage} selectedPlayer={selectedPlayer} />} />
          <Route path="controlpanel/" element={<ControlPanel data={data.controlPanel} tricksData={data.tricksPage} selectedPlayer={selectedPlayer} />} />
          <Route path="solver/" element={<SolverPage data={data.solverPage} />} />
        </Route>
      </Route>
    </Routes>
  );
}

function Root({ isConnected }: { isConnected: boolean }) {
  return (
    <>
      <Navbar>
        <Navbar.Brand>
          <Link to={`/`}><h1>🚀</h1></Link>
        </Navbar.Brand>
        <Nav variant="tabs">
          <Nav.Item>
            <Nav.Link>
              <Link to={`/user/`}>User</Link>
            </Nav.Link>
          </Nav.Item>
          <Nav.Item>
            <Nav.Link>
              <Link to={`/hand/`}>Hand</Link>
            </Nav.Link>
          </Nav.Item>
          <Nav.Item>
            <Nav.Link>
              <Link to={`/objectives/`}>Objectives</Link>
            </Nav.Link>
          </Nav.Item>
          <Nav.Item>
            <Nav.Link>
              <Link to={`/tricks/`}>Tricks</Link>
            </Nav.Link>
          </Nav.Item>
          <Nav.Item>
            <Nav.Link>
              <Link to={`/controlpanel/`}>Control Panel</Link>
            </Nav.Link>
          </Nav.Item>
          <Nav.Item>
            <Nav.Link>
              <Link to={`/solver/`}>Solver</Link>
            </Nav.Link>
          </Nav.Item>
          <Nav.Item>
            <Navbar.Text>
              {isConnected ? "Connected" : "Disconnected"}
            </Navbar.Text>
          </Nav.Item>
        </Nav>
      </Navbar >
      <Container>
        <Outlet />
      </Container>
    </>
  );
}

function IndexPage() {
  return <h1>🌎✨🚀✨🪐</h1>
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
