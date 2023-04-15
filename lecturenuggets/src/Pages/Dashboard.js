import React, { useEffect, useState } from 'react';
import { Container, Row, Col } from 'react-bootstrap';
import 'bootstrap/dist/css/bootstrap.min.css';

import styles from './Dashboard.module.css';

const Dashboard = () => {
	const [videos, setVideos] = useState(Array.from({ length: 6 }));

	const loadMoreVideos = () => {
		setVideos((prevVideos) => [...prevVideos, ...Array.from({ length: 6 })]);
	};

	useEffect(() => {
		const handleScroll = () => {
			if (
				window.innerHeight + document.documentElement.scrollTop !==
				document.documentElement.offsetHeight
			)
				return;
			loadMoreVideos();
		};
		window.addEventListener('scroll', handleScroll);
		return () => window.removeEventListener('scroll', handleScroll);
	}, []);

	return (
		<div className={styles.dashboard}>
			<div className={styles.leftSidebar}></div>
			<div className={styles.mainContent}>
				<br />
				<br />
				<br />
				<Container>
					<Row>
						<Col xs={12} className={styles.fullVideo}>
							<VideoCard index={0} key={0} />
						</Col>
						{videos.slice(1).map((_, index) => (
							<VideoCard index={index + 1} key={index + 1} />
						))}
					</Row>
				</Container>
			</div>
			<div className={styles.rightSidebar}></div>
		</div>
	);
};

const VideoCard = ({ index }) => {
	return (
		<Col xs={12} sm={6} md={4} key={index} className={styles.videoCard}>
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
					</div>
				</div>
			</div>
		</Col>
	);
};

export default Dashboard;
