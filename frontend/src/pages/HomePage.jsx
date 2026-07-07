import React from 'react';
import { Link } from 'react-router-dom';
import { motion } from 'framer-motion';
import '../styles/HomePage.css';

export default function HomePage() {
  const containerVariants = {
    hidden: { opacity: 0 },
    visible: {
      opacity: 1,
      transition: {
        staggerChildren: 0.1,
      },
    },
  };

  const itemVariants = {
    hidden: { opacity: 0, y: 20 },
    visible: { opacity: 1, y: 0 },
  };

  return (
    <motion.div
      className="home-page"
      initial="hidden"
      animate="visible"
      variants={containerVariants}
    >
      <div className="hero-section">
        <motion.div className="hero-content" variants={itemVariants}>
          <h1>🌍 Language Learning Game</h1>
          <p>Learn Russian, Turkish, Korean & English in a fun and interactive way</p>
          <div className="hero-buttons">
            <Link to="/login" className="btn btn-primary">
              Start Learning
            </Link>
            <Link to="/register" className="btn btn-secondary">
              Join Now
            </Link>
          </div>
        </motion.div>

        <motion.div className="hero-image" variants={itemVariants}>
          <div className="character-preview">👨‍🏫</div>
        </motion.div>
      </div>

      <div className="features-section">
        <h2>Why Learn with Us?</h2>
        <div className="features-grid">
          <motion.div className="feature-card" variants={itemVariants}>
            <div className="feature-icon">🎮</div>
            <h3>Gamified Learning</h3>
            <p>Learn through interactive games and challenges</p>
          </motion.div>

          <motion.div className="feature-card" variants={itemVariants}>
            <div className="feature-icon">👥</div>
            <h3>Interactive Characters</h3>
            <p>Chat with AI characters and practice conversations</p>
          </motion.div>

          <motion.div className="feature-card" variants={itemVariants}>
            <div className="feature-icon">🏆</div>
            <h3>Leaderboards</h3>
            <p>Compete with other learners worldwide</p>
          </motion.div>

          <motion.div className="feature-card" variants={itemVariants}>
            <div className="feature-icon">📊</div>
            <h3>Progress Tracking</h3>
            <p>Monitor your learning journey with detailed stats</p>
          </motion.div>
        </div>
      </div>

      <div className="languages-section">
        <h2>Available Languages</h2>
        <div className="languages-grid">
          <div className="language-card">
            <span className="flag">🇷🇺</span>
            <h3>Russian</h3>
          </div>
          <div className="language-card">
            <span className="flag">🇹🇷</span>
            <h3>Turkish</h3>
          </div>
          <div className="language-card">
            <span className="flag">🇬🇧</span>
            <h3>English</h3>
          </div>
          <div className="language-card">
            <span className="flag">🇰🇷</span>
            <h3>Korean</h3>
          </div>
        </div>
      </div>
    </motion.div>
  );
}
