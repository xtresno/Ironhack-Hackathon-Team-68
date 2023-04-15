import React from 'react';
import 'bootstrap/dist/css/bootstrap.min.css';
import { Button, Navbar, Container, Form, FormControl, Nav } from 'react-bootstrap';
import styles from './Navbar.css';

import lecturenuggetspng from "./icons/Lecturenuggets.png";

import { Link } from 'react-router-dom';


const Navbar_Component = () => {
    return (
        <Navbar className="MY_NAV fixed-top me-auto" expand="lg" bg="dark" variant="dark">
            <Container className="navbar-container">


                <Navbar.Toggle aria-controls="basic-navbar-nav" />

                <Navbar.Collapse id="basic-navbar-nav" className="justify-content-end">

                    <Form className="d-flex">
                        <FormControl
                            type="search"
                            placeholder="Search"
                            className="mr-2"
                            aria-label="Search"
                        />
                    </Form>

                    <Nav className="ms-auto">


                        <Nav.Link>
                            <Button className="nav_login btn-dark shadow">Login</Button>
                        </Nav.Link>


                        <Nav.Link>
                            <Button className="nav_login btn-dark shadow">Language</Button>
                        </Nav.Link>


                        <Nav.Link as={Link} to="/teacherdashboard">
                            <Button className="nav_upload btn-primary shadow lg">Upload</Button>
                        </Nav.Link>



                    </Nav>
                </Navbar.Collapse>
            </Container>
        </Navbar>
    );
};

export default Navbar_Component;
