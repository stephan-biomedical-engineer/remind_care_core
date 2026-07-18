<script setup>
import { onMounted, ref } from 'vue'

const showVideo = ref(false)

onMounted(() => {
  const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        entry.target.classList.add('visible')
      }
    })
  }, { threshold: 0.1 })

  document.querySelectorAll('.fade-up').forEach((el) => {
    observer.observe(el)
  })
})
</script>

<template>
  <div class="app-wrapper">
    <!-- Background Effects -->
    <div class="background-effects">
      <div class="glow-orb orb-primary"></div>
      <div class="glow-orb orb-secondary"></div>
      <div class="glow-orb orb-accent"></div>
      <div class="grid-overlay"></div>
    </div>

    <!-- Navbar -->
    <nav class="navbar">
      <div class="container nav-content">
        <div class="logo">
          <div class="logo-icon"></div>
          RemindCare
        </div>
        <div class="nav-links">
          <a href="#solucao">Visão Geral</a>
          <a href="#hardware">A Caixa Inteligente</a>
          <a href="#app">O Aplicativo</a>
          <a href="remindcare-release.apk?v=2" download class="btn-login">Baixar App (.APK) <span class="arrow">↓</span></a>
        </div>
      </div>
    </nav>

    <!-- Hero Section -->
    <header id="solucao" class="hero">
      <div class="container hero-container">
        <div class="hero-text fade-up">
          <div class="badge">Lançamento Exclusivo</div>
          <h1>O Futuro do <br>Cuidado está <br><span class="text-gradient">Sincronizado.</span></h1>
          <p>Esqueça os alarmes frustrantes e as planilhas manuais. A RemindCare une Hardware Premium e Software em Nuvem para garantir 100% de adesão ao tratamento médico da sua família.</p>
          
          <div class="cta-group">
            <a href="remindcare-release.apk?v=2" download class="btn-primary glow-effect">
              Baixar App Android (.APK)
            </a>
            <a href="#" @click.prevent="showVideo = true" class="btn-secondary outline"><span class="play-icon">▶</span> Ver Demonstração</a>
          </div>
          
          <div class="stats">
            <div class="stat-item">
              <strong>99.8%</strong>
              <span>Uptime da Nuvem</span>
            </div>
            <div class="stat-item">
              <strong>< 50ms</strong>
              <span>Latência de Notificação</span>
            </div>
          </div>
        </div>
        
        <div class="hero-visual fade-up">
          <div class="mockup-container floating">
            <img src="/app.png" alt="Aplicativo RemindCare" class="mockup-img" />
            <div class="glass-badge floating-reverse">
              <span>🔒 Criptografia End-to-End</span>
            </div>
          </div>
        </div>
      </div>
    </header>

    <!-- Hardware Section -->
    <section id="hardware" class="hardware-section">
      <div class="container">
        <div class="section-heading text-center fade-up">
          <h2>Conheça a Smart Pillbox</h2>
          <p>Design premium em glassmorphism com LEDs vibrantes que orientam o paciente visualmente.</p>
        </div>
        
        <div class="hardware-showcase fade-up">
          <img src="/pillbox.png" alt="Caixa Inteligente RemindCare" class="hardware-img" />
          <div class="hardware-overlay">
            <div class="feature-tag">Sincronização 4G/Wi-Fi</div>
            <div class="feature-tag right">Sensores Magnéticos de Abertura</div>
          </div>
        </div>

        <div class="hardware-details fade-up" style="margin-top: 4rem;">
          <h3 style="text-align: center; margin-bottom: 2rem; font-size: 2rem;">Como Funciona? (Manual Rápido)</h3>
          <div class="hardware-grid">
            <div class="hardware-card">
              <h4>1. Sincronização Automática</h4>
              <p>Ao ser ligada na tomada, a caixa se conecta à rede Wi-Fi e baixa toda a agenda médica cadastrada no aplicativo, salvando-a no seu cérebro interno.</p>
            </div>
            <div class="hardware-card">
              <h4>2. Alertas Visuais</h4>
              <p>No horário exato de cada remédio, o LED correspondente à gaveta do medicamento pisca, orientando exatamente o que deve ser tomado.</p>
            </div>
            <div class="hardware-card">
              <h4>3. Resiliência Offline</h4>
              <p>Se a internet cair, a caixa usa seu relógio interno. Ela continua funcionando e armazena os eventos localmente para enviá-los à nuvem depois.</p>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- App Section -->
    <section id="app" class="features-section">
      <div class="container lifestyle-container">
        <div class="lifestyle-visual fade-up">
           <img src="/lifestyle.png" alt="Idosa usando o RemindCare" class="lifestyle-img" />
        </div>
        <div class="lifestyle-text fade-up">
          <h2>Tranquilidade para quem cuida. <br> Independência para quem usa.</h2>
          <p>O ecossistema RemindCare foi desenhado para ser invísivel, porém implacável. Se o horário passar e a caixa não for aberta, notificações Push de alta prioridade são disparadas imediatamente para o celular dos familiares responsáveis.</p>
          <ul class="feature-list">
            <li>✅ Avisos instantâneos de atraso</li>
            <li>✅ Relatórios semanais de adesão (PDF)</li>
            <li>✅ Proteção contra doses antecipadas/duplicadas</li>
          </ul>
        </div>
      </div>
    </section>

    <!-- Footer -->
    <footer class="footer">
      <div class="container footer-content">
        <div class="footer-brand">
          <div class="logo">RemindCare</div>
          <p>O ecossistema definitivo para a gestão médica inteligente.</p>
        </div>
        <div class="footer-links">
          <p>&copy; 2026 RemindCare Systems. Desenvolvido com Rust 🦀 e Flutter 💙.</p>
        </div>
      </div>
    </footer>
    <!-- Video Modal -->
    <div class="video-modal" :class="{ 'is-active': showVideo }" @click="showVideo = false">
      <div class="modal-content" @click.stop>
        <button class="close-btn" @click="showVideo = false">&times;</button>
        <div class="video-container" v-if="showVideo">
          <iframe 
            width="100%" 
            height="100%" 
            src="https://www.youtube.com/embed/8ZuCT4PmurY?autoplay=1" 
            title="YouTube video player" 
            frameborder="0" 
            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" 
            allowfullscreen>
          </iframe>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
/* CSS Imports from the old style with adjustments for Vue */
:root {
  --bg-primary: #0a0a0c;
  --bg-secondary: #121217;
  --text-primary: #f4f4f5;
  --text-secondary: #a1a1aa;
  --accent-blue: #3b82f6;
  --accent-purple: #8b5cf6;
  --accent-cyan: #06b6d4;
  --accent-green: #10b981;
  --glass-bg: rgba(255, 255, 255, 0.03);
  --glass-border: rgba(255, 255, 255, 0.08);
  --glass-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.4);
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  line-height: 1.6;
  overflow-x: hidden;
}

html {
  scroll-behavior: smooth;
}

h1, h2, h3, h4, .logo {
  font-family: 'Outfit', sans-serif;
}

a {
  text-decoration: none;
  color: inherit;
}

/* Background Effects */
.background-effects {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: -1;
  overflow: hidden;
}

.glow-orb {
  position: absolute;
  border-radius: 50%;
  filter: blur(100px);
  opacity: 0.4;
  animation: float 20s infinite alternate;
}

.orb-primary {
  top: -10%;
  right: -5%;
  width: 50vw;
  height: 50vw;
  background: radial-gradient(circle, rgba(139,92,246,0.3) 0%, rgba(0,0,0,0) 70%);
}

.orb-secondary {
  bottom: -10%;
  left: -10%;
  width: 40vw;
  height: 40vw;
  background: radial-gradient(circle, rgba(59,130,246,0.2) 0%, rgba(0,0,0,0) 70%);
}

.orb-accent {
  top: 40%;
  left: 40%;
  width: 30vw;
  height: 30vw;
  background: radial-gradient(circle, rgba(6,182,212,0.15) 0%, rgba(0,0,0,0) 70%);
  animation-delay: -5s;
}

.grid-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-image: 
      linear-gradient(to right, rgba(255,255,255,0.02) 1px, transparent 1px),
      linear-gradient(to bottom, rgba(255,255,255,0.02) 1px, transparent 1px);
  background-size: 50px 50px;
  mask-image: radial-gradient(circle at center, black 40%, transparent 100%);
  -webkit-mask-image: radial-gradient(circle at center, black 40%, transparent 100%);
}

/* Container */
.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 2rem;
}

/* Navbar */
.navbar {
  padding: 1.5rem 0;
  position: fixed;
  top: 0;
  width: 100%;
  z-index: 100;
  background: rgba(10, 10, 12, 0.7);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border-bottom: 1px solid var(--glass-border);
}

.nav-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.logo {
  font-size: 1.5rem;
  font-weight: 800;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.logo span {
  color: var(--accent-blue);
}

.logo-icon {
  width: 24px;
  height: 24px;
  background: linear-gradient(135deg, var(--accent-blue), var(--accent-purple));
  border-radius: 6px;
}

.nav-links {
  display: flex;
  gap: 2.5rem;
  align-items: center;
}

.nav-links a:not(.btn-login) {
  font-size: 0.95rem;
  font-weight: 500;
  color: var(--text-secondary);
  transition: color 0.2s;
}

.nav-links a:not(.btn-login):hover {
  color: var(--text-primary);
}

/* Buttons */
.btn-login {
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  padding: 0.6rem 1.2rem;
  border-radius: 8px;
  font-weight: 600;
  font-size: 0.9rem;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.btn-login:hover {
  background: rgba(255, 255, 255, 0.15);
  transform: translateY(-2px);
}

.btn-primary {
  background: linear-gradient(135deg, #10b981, #059669);
  color: white;
  padding: 1rem 2rem;
  border-radius: 12px;
  font-weight: 600;
  font-size: 1.05rem;
  display: inline-block;
  transition: all 0.3s ease;
  border: none;
  box-shadow: 0 4px 15px rgba(16, 185, 129, 0.4);
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(16, 185, 129, 0.6);
}

.btn-secondary {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: var(--text-primary);
  padding: 1rem 2rem;
  border-radius: 12px;
  font-weight: 600;
  font-size: 1.05rem;
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.3s ease;
}

.btn-secondary:hover {
  background: rgba(255, 255, 255, 0.1);
  transform: translateY(-2px);
}

/* Hero Section */
.hero {
  padding-top: 150px;
  padding-bottom: 80px;
  min-height: 100vh;
  display: flex;
  align-items: center;
}

.hero-container {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 4rem;
  align-items: center;
}

.badge {
  display: inline-block;
  padding: 0.4rem 1rem;
  background: rgba(139, 92, 246, 0.1);
  border: 1px solid rgba(139, 92, 246, 0.3);
  color: #c4b5fd;
  border-radius: 20px;
  font-size: 0.85rem;
  font-weight: 600;
  margin-bottom: 1.5rem;
}

.hero-text h1 {
  font-size: 4rem;
  line-height: 1.1;
  margin-bottom: 1.5rem;
  letter-spacing: -0.02em;
}

.text-gradient {
  background: linear-gradient(135deg, #60a5fa, #c084fc);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.hero-text p {
  font-size: 1.15rem;
  color: var(--text-secondary);
  margin-bottom: 2.5rem;
  max-width: 90%;
}

.cta-group {
  display: flex;
  gap: 1rem;
  margin-bottom: 3rem;
}

.stats {
  display: flex;
  gap: 3rem;
  border-top: 1px solid var(--glass-border);
  padding-top: 2rem;
}

.stat-item {
  display: flex;
  flex-direction: column;
}

.stat-item strong {
  font-size: 1.8rem;
  font-family: 'Outfit', sans-serif;
  color: var(--text-primary);
}

.stat-item span {
  font-size: 0.85rem;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  font-weight: 500;
}

.mockup-container {
  position: relative;
}

.mockup-img {
  width: 100%;
  border-radius: 24px;
  box-shadow: var(--glass-shadow);
  border: 1px solid var(--glass-border);
}

.hardware-section {
  padding: 100px 0;
  position: relative;
}

.hardware-showcase {
  position: relative;
  max-width: 900px;
  margin: 0 auto;
  border-radius: 24px;
  overflow: hidden;
  box-shadow: 0 20px 50px rgba(0,0,0,0.5);
  border: 1px solid var(--glass-border);
}

.hardware-img {
  width: 100%;
  display: block;
}

.feature-tag {
  position: absolute;
  top: 20%;
  left: -20px;
  background: rgba(10, 10, 12, 0.8);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  padding: 1rem 1.5rem;
  border-radius: 16px;
  font-weight: 600;
  font-size: 0.9rem;
  box-shadow: 0 10px 30px rgba(0,0,0,0.5);
  z-index: 20;
}

.feature-tag.right {
  left: auto;
  right: -20px;
  top: 60%;
}

.lifestyle-container {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 4rem;
  align-items: center;
  padding: 100px 2rem;
}

.lifestyle-img {
  width: 100%;
  border-radius: 24px;
  box-shadow: var(--glass-shadow);
  border: 1px solid var(--glass-border);
}

.feature-list {
  list-style: none;
  margin-top: 2rem;
}

.feature-list li {
  margin-bottom: 1rem;
  font-size: 1.1rem;
  font-weight: 500;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.glass-badge {
  position: absolute;
  bottom: -20px;
  right: -20px;
  background: rgba(10, 10, 12, 0.8);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  padding: 1rem 1.5rem;
  border-radius: 16px;
  font-weight: 600;
  font-size: 0.9rem;
  box-shadow: 0 10px 30px rgba(0,0,0,0.5);
  z-index: 20;
}

.section-heading {
  margin-bottom: 4rem;
}

.section-heading h2 {
  font-size: 2.5rem;
  margin-bottom: 1rem;
}

.text-center {
  text-align: center;
}

/* Footer */
.footer {
  border-top: 1px solid var(--glass-border);
  padding: 4rem 0 2rem;
  margin-top: 4rem;
  background: rgba(0,0,0,0.2);
}

.footer-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 2rem;
}

.footer-brand p {
  color: var(--text-secondary);
  margin-top: 0.5rem;
  font-size: 0.9rem;
}

.footer-links p {
  color: rgba(255,255,255,0.4);
  font-size: 0.85rem;
}

/* Animations */
@keyframes float {
  0% { transform: translate(0, 0); }
  100% { transform: translate(5%, 5%); }
}

@keyframes float-y {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-15px); }
}

.floating {
  animation: float-y 6s ease-in-out infinite;
}
.floating-reverse {
  animation: float-y 7s ease-in-out infinite reverse;
}

/* Scroll Animations */
.fade-up {
  opacity: 0;
  transform: translateY(30px);
  transition: opacity 0.8s ease, transform 0.8s ease;
}

.fade-up.visible {
  opacity: 1;
  transform: translateY(0);
}

/* Hardware Grid */
.hardware-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 2rem;
  margin-top: 2rem;
}

.hardware-card {
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid var(--glass-border);
  padding: 2rem;
  border-radius: 16px;
  backdrop-filter: blur(10px);
  transition: transform 0.3s;
}

.hardware-card:hover {
  transform: translateY(-5px);
  background: rgba(255, 255, 255, 0.05);
}

.hardware-card h4 {
  color: var(--accent-blue);
  margin-bottom: 1rem;
  font-size: 1.2rem;
}

.hardware-card p {
  color: var(--text-secondary);
  font-size: 0.95rem;
}

/* Video Modal */
.video-modal {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.85);
  backdrop-filter: blur(8px);
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  visibility: hidden;
  transition: opacity 0.3s, visibility 0.3s;
}

.video-modal.is-active {
  opacity: 1;
  visibility: visible;
}

.modal-content {
  width: 90%;
  max-width: 1000px;
  background: #000;
  border-radius: 16px;
  overflow: hidden;
  position: relative;
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
  border: 1px solid rgba(255, 255, 255, 0.1);
  transform: scale(0.95);
  transition: transform 0.3s;
}

.video-modal.is-active .modal-content {
  transform: scale(1);
}

.close-btn {
  position: absolute;
  top: 10px;
  right: 15px;
  background: none;
  border: none;
  color: white;
  font-size: 2rem;
  cursor: pointer;
  z-index: 10;
  text-shadow: 0 2px 4px rgba(0,0,0,0.5);
}

.close-btn:hover {
  color: var(--accent-blue);
}

.video-container {
  position: relative;
  padding-bottom: 56.25%; /* 16:9 Aspect Ratio */
  height: 0;
}

.video-container iframe {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}

/* Responsive */
@media (max-width: 992px) {
  .hero-container, .lifestyle-container {
    grid-template-columns: 1fr;
    text-align: center;
  }
  
  .hero-text p {
    margin: 0 auto 2.5rem;
  }
  
  .cta-group {
    justify-content: center;
  }
  
  .stats {
    justify-content: center;
  }
  
  .feature-list {
    text-align: left;
    display: inline-block;
  }
}

@media (max-width: 768px) {
  .hero-text h1 {
    font-size: 3rem;
  }
  .nav-links {
    display: none;
  }
}
</style>
