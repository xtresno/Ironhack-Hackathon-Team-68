import React from 'react';
import 'bootstrap/dist/css/bootstrap.min.css';
import { Button, Navbar, Container } from 'react-bootstrap';
import Nav from 'react-bootstrap/Nav';

// images

const Navbar_Component = () => {
    return (
        <Navbar className="MY_NAV fixed-top me-auto" expand="lg" bg="dark" variant="dark">
            <Container className="navbar-container">
                <Navbar.Brand className="nav-header text-white" href="/">
                    <img className="" alt="" />LectureNuggets.com
                </Navbar.Brand>

                <Navbar.Toggle aria-controls="basic-navbar-nav" />

                <Navbar.Collapse id="basic-navbar-nav" className="justify-content-end">
                    <Nav className="ms-auto">
                        <Nav.Link>
                            <Button className="nav_get_started btn-dark shadow">Get started</Button>
                        </Nav.Link>
                    </Nav>
                </Navbar.Collapse>
            </Container>
        </Navbar>
    );
};

export default Navbar_Component;
