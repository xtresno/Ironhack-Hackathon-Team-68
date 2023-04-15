import React from 'react';
import 'bootstrap/dist/css/bootstrap.min.css';
import './TeacherDashboard.css';
import classroomBackgroundImage from "./icons/classroom.jpg";
import lecturenuggetspng from "./icons/Lecturenuggets.png";
import { Link } from "react-router-dom";

const TeacherDashboard = () => {
    return (
        <div>
            {/* Add the navbar with the logo */}
            <nav className="navbar">
                <div className="logo">
                    {/* Wrap the logo inside a Link component */}
                    <Link to="/">
                        <img className="nav_logo_header image_white" src={lecturenuggetspng} alt="" />
                    </Link>
                </div>
            </nav>
            {/* Add the content of the Teacher Dashboard */}
            <div className="image-container">
                <div className="image-border">
                    <img src={classroomBackgroundImage} className="background-image" alt="Classroom Background" />
                    <form>
                        <button type="submit" className="button_teacher_upload">Upload Lectures</button>
                    </form>
                </div>
                <div className="image-border">
                    <img src={classroomBackgroundImage} className="background-image" alt="Classroom Background" />
                </div>
            </div>
        </div>
    );
};

export default TeacherDashboard;
