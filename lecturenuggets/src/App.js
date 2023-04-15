import './App.css';
import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Dashboard from './Pages/Dashboard';
import TeacherDashboard from './Pages/TeacherDashboard';

import Navbar from './Navigation/Nav_Comp.js';


const App = () => {
  return (
    <div>
      <Router>
        <Navbar />
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/teacherdashboard" element={<TeacherDashboard />} />

        </Routes>
      </Router>
    </div>
  );
};

export default App;
