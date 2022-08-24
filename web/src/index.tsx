import * as React from "react";
import * as ReactDOM from "react-dom/client";

import "./index.scss";

import Application from "./components/app";

const root = ReactDOM.createRoot(document.getElementById("application")!);
root.render(<Application />);
