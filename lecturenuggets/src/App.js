import './App.css';
import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Dashboard from './Pages/Dashboard';
import Navbar from './Navigation/Nav_Comp.js';

const App = () => {
  return (
    <div>
      <Router>
        <Navbar />
        <Routes>
          {/* Uncomment the line below when you have a Home component */}
          {/* <Route path="/" element={<Home />} /> */}
          <Route path="/" element={<Dashboard />} />
        </Routes>
      </Router>
    </div>
  );
};

export default App;
