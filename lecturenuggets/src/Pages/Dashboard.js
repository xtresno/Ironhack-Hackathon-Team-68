import React from 'react';
import { Container, Row, Col, Form, InputGroup } from 'react-bootstrap';
import 'bootstrap/dist/css/bootstrap.min.css';
import styles from './Dashboard.module.css';

import { Heart, ChatDots, Share } from 'react-bootstrap-icons';

import { Link } from "react-router-dom"; // Import the Link component from react-router-dom


import lecturenuggetspng from "./icons/Lecturenuggets.png";


const Dashboard = () => {
	return (
		<div className={styles.dashboard}>
			<div className={styles.sideNav}>
				<Link className="logo" to="/">
					<img className="nav_logo_header image_white" src={lecturenuggetspng} alt="" />
				</Link>

				<h4 className='ageContainer'>Filter by Age</h4>
				{Array.from({ length: 5 }, (_, i) => i + 1).map((age) => (
					<Form.Check type="checkbox" key={age} label={`Age ${age}`} className={styles.ageCheckbox} />
				))}
			</div>
			<div className={styles.mainContent}>
				<Container fluid>
					<Row>
						<Col xs={12} className={styles.fullVideo}>
							<VideoCard index={0} key={0} />
						</Col>
						<Col xs={12} className={styles.fullThumbnail}>
							<Thumbnail index={1} />
						</Col>
					</Row>
				</Container>
			</div>
		</div>
	);
};



const Thumbnail = ({ index }) => {
	return (
		<div className={styles.video}>
			<img
				src={`https://via.placeholder.com/150?text=Thumbnail%20${index + 1}`}
				alt={`Video Thumbnail ${index + 1}`}
				className="img-fluid"
			/>
		</div>
	);
};



const VideoCard = ({ index }) => {
	return (
		<Col xs={12} key={index} className={styles.videoCard}>
			<div className={styles.video}>
				<img
					src={`https://via.placeholder.com/150?text=Thumbnail%20${index + 1}`}
					alt={`Video Thumbnail ${index + 1}`}
					className="img-fluid"
				/>
				<div className={styles.videoInfo}>
					<h4>Video Title {index + 1}</h4>
					<div className={styles.actions}>
						<button className="btn btn-primary">Play</button>
						<button className="btn btn-secondary">Learn More</button>
						<div className={styles.iconContainer}>
							<Heart className={styles.icon} />
							<ChatDots className={styles.icon} />
							<Share className={styles.icon} />
						</div>
					</div>
				</div>
			</div>
		</Col>
	);
};





export default Dashboard;
