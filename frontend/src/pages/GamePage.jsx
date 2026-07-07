import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import '../styles/GamePage.css';

export default function GamePage() {
  const [character, setCharacter] = useState(null);
  const [dialogue, setDialogue] = useState([]);
  const [userInput, setUserInput] = useState('');
  const [xp, setXp] = useState(0);
  const [level, setLevel] = useState(1);
  const [selectedLanguage, setSelectedLanguage] = useState('russian');

  const characters = {
    russian: { name: 'Aleksandr', emoji: '👨‍🏫', color: '#E74C3C' },
    turkish: { name: 'Ahmet', emoji: '👨‍🏫', color: '#F39C12' },
    english: { name: 'James', emoji: '👨‍🏫', color: '#3498DB' },
    korean: { name: 'Min-jun', emoji: '👨‍🏫', color: '#9B59B6' },
  };

  useEffect(() => {
    setCharacter(characters[selectedLanguage]);
    initializeDialogue();
  }, [selectedLanguage]);

  const initializeDialogue = () => {
    const greetings = {
      russian: 'Привет! Готов ли ты учиться?',
      turkish: 'Merhaba! Öğrenmeye hazır mısın?',
      english: 'Hello! Are you ready to learn?',
      korean: '안녕하세요! 배울 준비가 되셨나요?',
    };
    setDialogue([{ speaker: 'character', text: greetings[selectedLanguage] }]);
  };

  const handleSendMessage = () => {
    if (!userInput.trim()) return;

    // Add user message
    const newDialogue = [
      ...dialogue,
      { speaker: 'user', text: userInput },
    ];

    // Simulate character response
    const responses = {
      russian: 'Отлично! Давай начнём с новых слов.',
      turkish: 'Harika! Yeni kelimelerle başlayalım.',
      english: 'Great! Let\'s start with new words.',
      korean: '좋습니다! 새로운 단어로 시작해봅시다.',
    };

    setTimeout(() => {
      newDialogue.push({ speaker: 'character', text: responses[selectedLanguage] });
      setDialogue(newDialogue);
    }, 500);

    setUserInput('');
    setXp(xp + 10);
  };

  return (
    <div className="game-page">
      <div className="game-header">
        <div className="stats">
          <div className="stat-item">
            <span className="stat-label">Level</span>
            <span className="stat-value">{level}</span>
          </div>
          <div className="stat-item">
            <span className="stat-label">XP</span>
            <span className="stat-value">{xp}</span>
          </div>
        </div>

        <div className="language-selector">
          {Object.entries(characters).map(([key, char]) => (
            <button
              key={key}
              className={`language-btn ${selectedLanguage === key ? 'active' : ''}`}
              onClick={() => setSelectedLanguage(key)}
            >
              {char.name}
            </button>
          ))}
        </div>
      </div>

      <div className="game-container">
        <div className="character-section">
          <motion.div
            className="character-display"
            animate={{ y: [0, -10, 0] }}
            transition={{ duration: 2, repeat: Infinity }}
          >
            <div className="character-emoji">{character?.emoji}</div>
            <h2>{character?.name}</h2>
          </motion.div>

          <motion.div className="character-bubble">
            <AnimatePresence>
              {dialogue.length > 0 && (
                <motion.p
                  key={dialogue[dialogue.length - 1].text}
                  initial={{ opacity: 0, scale: 0.8 }}
                  animate={{ opacity: 1, scale: 1 }}
                  exit={{ opacity: 0, scale: 0.8 }}
                >
                  {dialogue[dialogue.length - 1].text}
                </motion.p>
              )}
            </AnimatePresence>
          </motion.div>
        </div>

        <div className="dialogue-section">
          <div className="dialogue-history">
            <AnimatePresence>
              {dialogue.map((msg, idx) => (
                <motion.div
                  key={idx}
                  className={`dialogue-message ${msg.speaker}`}
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -10 }}
                >
                  <p>{msg.text}</p>
                </motion.div>
              ))}
            </AnimatePresence>
          </div>

          <div className="input-section">
            <input
              type="text"
              value={userInput}
              onChange={(e) => setUserInput(e.target.value)}
              onKeyPress={(e) => e.key === 'Enter' && handleSendMessage()}
              placeholder="Type your response..."
              className="message-input"
            />
            <button onClick={handleSendMessage} className="send-btn">
              Send
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
