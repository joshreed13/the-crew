import React from 'react';
import {
  createBrowserRouter,
  createRoutesFromElements,
  RouterProvider,
  useRouteError,
  Route,
  Outlet,
  Link,
} from "react-router-dom"
import './App.css';
import ControlPanel from './ControlPanel';
import { Card } from './model';

const mycards: Card[] = [
  { suit: "B", value: 1 },
  { suit: "Y", value: 3 },
  { suit: "M", value: 6 },
  { suit: "G", value: 9 },
  { suit: "R", value: 4 }
];

const router = createBrowserRouter(
  createRoutesFromElements(
    <Route path="/" element={<Root />} errorElement={<ErrorPage />}>
      <Route errorElement={<ErrorPage />}>
        <Route index element={<IndexPage />} />
        <Route path="controlpanel/" element={<ControlPanel cards={mycards} />} />
      </Route>
    </Route >
  )
);

function App() {
  return (
    <RouterProvider router={router} />
  );
}

function Root() {
  return (
    <>
      <div id="sidebar">
        <h1>🚀</h1>
        <nav>
          <ul>
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
