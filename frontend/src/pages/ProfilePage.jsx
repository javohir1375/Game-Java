import React, { useState, useEffect } from 'react';
import { motion } from 'framer-motion';
import axios from 'axios';
import '../styles/ProfilePage.css';

export default function ProfilePage({ user }) {
  const [stats, setStats] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchStats();
  }, []);

  const fetchStats = async () => {
    try {
      const response = await axios.get(
        `http://localhost:8000/api/users/${user?.id}/stats`,
        {
          headers: {
            Authorization: `Bearer ${localStorage.getItem('token')}`,
          },
        }
      );
      setStats(response.data);
    } catch (error) {
      console.error('Failed to fetch stats:', error);
    } finally {
      setLoading(false);
    }
  };

  if (loading) return <div className="loading">Loading...</div>;

  return (
    <motion.div
      className="profile-page"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.5 }}
    >
      <div className="profile-container">
        <div className="profile-header">
          <div className="profile-avatar">👤</div>
          <h1>{user?.username}</h1>
          <p>Level {user?.level}</p>
        </div>

        <div className="profile-stats">
          <div className="stat-card">
            <h3>📚 Lessons</h3>
            <p>{stats?.lessons_completed || 0}</p>
          </div>
          <div className="stat-card">
            <h3>⚡ Exercises</h3>
            <p>{stats?.exercises_solved || 0}</p>
          </div>
          <div className="stat-card">
            <h3>🔥 Streak</h3>
            <p>{stats?.current_streak || 0} days</p>
          </div>
          <div className="stat-card">
            <h3>⭐ Total XP</h3>
            <p>{stats?.total_xp || 0}</p>
          </div>
        </div>

        <div className="languages-progress">
          <h2>Language Progress</h2>
          {stats?.languages?.map((lang) => (
            <div key={lang.language} className="language-progress">
              <h3>{lang.language}</h3>
              <div className="progress-bar">
                <div
                  className="progress-fill"
                  style={{ width: `${lang.progress}%` }}
                ></div>
              </div>
              <p>{lang.words_learned} words learned</p>
            </div>
          ))}
        </div>
      </div>
    </motion.div>
  );
}
