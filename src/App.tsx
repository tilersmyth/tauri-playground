import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import { HomePage } from "./pages/home";
import { AdminPage } from "./pages/admin";
import { AuthRoutes, NonAuthRoutes } from "./auth";

const App = () => (
  <Router>
    <Routes>
      <Route element={<NonAuthRoutes />}>
        <Route path="/" element={<HomePage />} />
      </Route>
      <Route element={<AuthRoutes />}>
        <Route path="/admin" element={<AdminPage />} />
      </Route>
    </Routes>
  </Router>
);

export default App;
