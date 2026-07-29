<script setup>
import { onMounted, ref } from 'vue'

const showVideo = ref(false)
const activeTab = ref('wifi')
const previewImage = ref(null)

const manualSteps = {
  wifi: {
    title: '1. Conexão Wi-Fi do Hardware',
    subtitle: 'Conecte a caixa inteligente à sua rede doméstica em poucos passos',
    steps: [
      { id: 1, title: 'Rede RemindCare', desc: 'Ao ligar a caixa, selecione a rede Wi-Fi RemindCare-Caixa no celular.', img: '/tela1.jpg' },
      { id: 2, title: 'Portal Automático', desc: 'O portal captive de configuração abrirá automaticamente na tela.', img: '/tela2.jpg' },
      { id: 3, title: 'Credenciais de Casa', desc: 'Selecione sua rede Wi-Fi doméstica e digite a senha correspondente.', img: '/tela3.jpg' },
      { id: 4, title: 'Conexão Nuvem', desc: 'A caixinha se conecta à internet e confirma a prontidão de uso.', img: '/tela4.jpg' }
    ]
  },
  app: {
    title: '2. Cadastro e Pareamento',
    subtitle: 'Crie sua conta de usuário e vincule o aplicativo à caixinha inteligente',
    steps: [
      { id: 5, title: 'Criar Conta', desc: 'Cadastre seus dados de paciente ou cuidador no aplicativo.', img: '/tela5.jpg' },
      { id: 6, title: 'Login Seguro', desc: 'Acesse o aplicativo com suas credenciais de segurança.', img: '/tela6.jpg' },
      { id: 7, title: 'Buscar Dispositivo', desc: 'Toque no botão de parear caixinha no aplicativo para localizar o hardware.', img: '/tela7.jpg' },
      { id: 8, title: 'Status Online (Verde)', desc: 'O indicador com a bolinha verde confirma que a caixinha está vinculada.', img: '/tela8.jpg' }
    ]
  },
  remedios: {
    title: '3. Medicamentos e Compartimentos',
    subtitle: 'Cadastre a receita médica, defina os alertas e associe às gavetas com LED',
    steps: [
      { id: 9, title: 'Novo Remédio', desc: 'Toque no botão de adicionar medicamento para iniciar o cadastro.', img: '/tela9.jpg' },
      { id: 10, title: 'Horários e Dosagem', desc: 'Configure o nome, intervalo de horas e escolha o compartimento da caixa.', img: '/tela10.jpg' },
      { id: 11, title: 'Grade Semanal', desc: 'Acompanhe a agenda semanal completa com os horários de cada dose.', img: '/tela11.jpg' },
      { id: 12, title: 'Gavetas & LEDs', desc: 'Veja a representação gráfica das gavetas e seus respectivos LEDs.', img: '/tela12.jpg' }
    ]
  },
  relatorios: {
    title: '4. Monitoramento e Relatório PDF',
    subtitle: 'Acompanhe o tratamento em tempo real e exporte relatórios para o seu médico',
    steps: [
      { id: 13, title: 'Doses Pendentes & Stats', desc: 'Monitore os remédios a serem tomados e o índice diário de adesão.', img: '/tela13.jpg' },
      { id: 14, title: 'Relatório em PDF', desc: 'Exporte relatórios em PDF com o histórico detalhado para apresentar na consulta.', img: '/tela14.jpg' }
    ]
  }
}

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
        <div class="logo">RemindCare</div>
        <div class="logo">
          <img src="/remindcare_logo.svg?v=2" alt="RemindCare Logo" style="height: 36px; object-fit: contain;" />
        </div>
        
        <div class="nav-links">
          <a href="#solucao">Visão Geral</a>
          <a href="#hardware">A Caixa Inteligente</a>
          <a href="#app">O Aplicativo</a>
        </div>

        <div class="nav-action">
          <a href="remindcare-release.apk?v=2" download class="btn-login">Android (.APK) <span class="arrow">↓</span></a>
          <a href="remindcare-release.ipa?v=2" download class="btn-login">iOS (.IPA) <span class="arrow">↓</span></a>
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
              Android (.APK) ↓
            </a>
            <a href="remindcare-release.ipa?v=2" download class="btn-primary btn-ios glow-effect">
              iOS (.IPA) ↓
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
            <img src="/remindcare_app.png" alt="Aplicativo RemindCare" class="mockup-img" />
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

        <!-- Interactive Visual Manual Section -->
        <div class="manual-section fade-up">
          <div class="manual-header text-center">
            <h3>Como Funciona? (Guia Visual Passo a Passo)</h3>
            <p>Acompanhe o fluxo completo desde a primeira conexão Wi-Fi até a emissão de relatórios médicos</p>
          </div>

          <!-- Navigation Tabs -->
          <div class="manual-tabs">
            <button 
              class="manual-tab-btn" 
              :class="{ active: activeTab === 'wifi' }"
              @click="activeTab = 'wifi'"
            >
              <span class="tab-num">1</span> Wi-Fi da Caixinha
            </button>
            <button 
              class="manual-tab-btn" 
              :class="{ active: activeTab === 'app' }"
              @click="activeTab = 'app'"
            >
              <span class="tab-num">2</span> Conta & Pareamento
            </button>
            <button 
              class="manual-tab-btn" 
              :class="{ active: activeTab === 'remedios' }"
              @click="activeTab = 'remedios'"
            >
              <span class="tab-num">3</span> Remédios & Gavetas
            </button>
            <button 
              class="manual-tab-btn" 
              :class="{ active: activeTab === 'relatorios' }"
              @click="activeTab = 'relatorios'"
            >
              <span class="tab-num">4</span> Alertas & PDF
            </button>
          </div>

          <!-- Tab Content Header -->
          <div class="tab-intro">
            <h4>{{ manualSteps[activeTab].title }}</h4>
            <p>{{ manualSteps[activeTab].subtitle }}</p>
          </div>

          <!-- Steps Grid -->
          <div class="steps-grid">
            <div 
              v-for="step in manualSteps[activeTab].steps" 
              :key="step.id" 
              class="step-card"
              @click="previewImage = step"
            >
              <div class="step-badge">Passo {{ step.id }}</div>
              <div class="step-image-wrapper">
                <img :src="step.img" :alt="step.title" class="step-img" />
                <div class="image-zoom-hint">🔍 Ampliar</div>
              </div>
              <div class="step-content">
                <h5>{{ step.title }}</h5>
                <p>{{ step.desc }}</p>
              </div>
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
          <div class="logo">
            <img src="/remindcare_logo.svg?v=2" alt="RemindCare Logo" style="height: 48px; object-fit: contain;" />
          </div>
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
            src="https://www.youtube.com/embed/fJRV5_p9IYU?autoplay=1" 
            title="YouTube video player" 
            frameborder="0" 
            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" 
            allowfullscreen>
          </iframe>
        </div>
      </div>
    </div>

    <!-- Image Zoom Modal -->
    <div class="image-modal" :class="{ 'is-active': previewImage }" @click="previewImage = null">
      <div class="image-modal-content" @click.stop v-if="previewImage">
        <button class="close-btn" @click="previewImage = null">&times;</button>
        <div class="modal-image-wrapper">
          <img :src="previewImage.img" :alt="previewImage.title" class="modal-img" />
        </div>
        <div class="modal-image-caption">
          <h4>Passo {{ previewImage.id }}: {{ previewImage.title }}</h4>
          <p>{{ previewImage.desc }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
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
  flex: 1;
  font-size: 1.5rem;
  font-weight: 800;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.nav-links {
  flex: 2;
  display: flex;
  justify-content: center;
  gap: 2.5rem;
  align-items: center;
}

.nav-action {
  flex: 1;
  display: flex;
  justify-content: flex-end;
  gap: 0.6rem;
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
  padding: 0.6rem 1rem;
  border-radius: 8px;
  font-weight: 600;
  font-size: 0.85rem;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 0.4rem;
  white-space: nowrap;
}

.btn-login:hover {
  background: rgba(255, 255, 255, 0.15);
  transform: translateY(-2px);
}

.btn-primary {
  background: linear-gradient(135deg, #10b981, #059669);
  color: white;
  padding: 1rem 1.8rem;
  border-radius: 12px;
  font-weight: 600;
  font-size: 1rem;
  display: inline-block;
  transition: all 0.3s ease;
  border: none;
  box-shadow: 0 4px 15px rgba(16, 185, 129, 0.4);
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(16, 185, 129, 0.6);
}

/* Estilo adicional para diferenciar o botão de iOS */
.btn-ios {
  background: linear-gradient(135deg, var(--accent-blue), var(--accent-purple));
  box-shadow: 0 4px 15px rgba(59, 130, 246, 0.4);
}

.btn-ios:hover {
  box-shadow: 0 8px 25px rgba(59, 130, 246, 0.6);
}

.btn-secondary {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: var(--text-primary);
  padding: 1rem 1.8rem;
  border-radius: 12px;
  font-weight: 600;
  font-size: 1rem;
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
  flex-wrap: wrap;
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

/* Interactive Visual Manual Styles */
.manual-section {
  margin-top: 4rem;
  padding: 3rem 2rem;
  background: rgba(18, 18, 23, 0.6);
  border-radius: 24px;
  border: 1px solid var(--glass-border);
  backdrop-filter: blur(16px);
  box-shadow: var(--glass-shadow);
}

.manual-header h3 {
  font-size: 2.2rem;
  margin-bottom: 0.5rem;
  background: linear-gradient(135deg, #f4f4f5, #a1a1aa);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.manual-header p {
  color: var(--text-secondary);
  font-size: 1.05rem;
  margin-bottom: 2.5rem;
}

.manual-tabs {
  display: flex;
  justify-content: center;
  gap: 1rem;
  flex-wrap: wrap;
  margin-bottom: 2.5rem;
}

.manual-tab-btn {
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid var(--glass-border);
  color: var(--text-secondary);
  padding: 0.8rem 1.4rem;
  border-radius: 14px;
  font-size: 0.95rem;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.6rem;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.manual-tab-btn .tab-num {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.8rem;
  color: var(--text-primary);
  transition: all 0.3s ease;
}

.manual-tab-btn:hover {
  background: rgba(255, 255, 255, 0.08);
  color: var(--text-primary);
  transform: translateY(-2px);
}

.manual-tab-btn.active {
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.2), rgba(139, 92, 246, 0.2));
  border-color: var(--accent-purple);
  color: var(--text-primary);
  box-shadow: 0 4px 20px rgba(139, 92, 246, 0.3);
}

.manual-tab-btn.active .tab-num {
  background: linear-gradient(135deg, var(--accent-blue), var(--accent-purple));
  color: #fff;
}

.tab-intro {
  text-align: center;
  margin-bottom: 2rem;
  padding-bottom: 1rem;
  border-bottom: 1px dashed var(--glass-border);
}

.tab-intro h4 {
  font-size: 1.4rem;
  color: var(--accent-cyan);
  margin-bottom: 0.3rem;
}

.tab-intro p {
  color: var(--text-secondary);
  font-size: 0.95rem;
}

.steps-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 1.5rem;
}

.step-card {
  background: rgba(255, 255, 255, 0.02);
  border: 1px solid var(--glass-border);
  border-radius: 18px;
  overflow: hidden;
  position: relative;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  flex-direction: column;
}

.step-card:hover {
  transform: translateY(-6px);
  background: rgba(255, 255, 255, 0.05);
  border-color: rgba(139, 92, 246, 0.4);
  box-shadow: 0 12px 30px rgba(0, 0, 0, 0.4);
}

.step-badge {
  position: absolute;
  top: 12px;
  left: 12px;
  z-index: 10;
  background: rgba(10, 10, 12, 0.85);
  backdrop-filter: blur(8px);
  border: 1px solid rgba(255, 255, 255, 0.15);
  color: var(--accent-blue);
  font-size: 0.75rem;
  font-weight: 700;
  padding: 0.3rem 0.7rem;
  border-radius: 20px;
}

.step-image-wrapper {
  position: relative;
  width: 100%;
  height: 280px;
  background: #000;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
}

.step-img {
  width: 100%;
  height: 100%;
  object-fit: contain;
  transition: transform 0.4s ease;
}

.step-card:hover .step-img {
  transform: scale(1.05);
}

.image-zoom-hint {
  position: absolute;
  bottom: 10px;
  right: 10px;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(6px);
  color: white;
  font-size: 0.75rem;
  padding: 0.3rem 0.6rem;
  border-radius: 8px;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.step-card:hover .image-zoom-hint {
  opacity: 1;
}

.step-content {
  padding: 1.2rem;
  flex: 1;
  display: flex;
  flex-direction: column;
}

.step-content h5 {
  font-size: 1.1rem;
  color: var(--text-primary);
  margin-bottom: 0.4rem;
}

.step-content p {
  font-size: 0.88rem;
  color: var(--text-secondary);
  line-height: 1.4;
}

/* Image Zoom Modal */
.image-modal {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.9);
  backdrop-filter: blur(12px);
  z-index: 2000;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  visibility: hidden;
  transition: opacity 0.3s, visibility 0.3s;
}

.image-modal.is-active {
  opacity: 1;
  visibility: visible;
}

.image-modal-content {
  max-width: 500px;
  width: 90%;
  max-height: 90vh;
  background: var(--bg-secondary);
  border: 1px solid var(--glass-border);
  border-radius: 24px;
  overflow: hidden;
  position: relative;
  box-shadow: var(--glass-shadow);
  display: flex;
  flex-direction: column;
}

.modal-image-wrapper {
  width: 100%;
  max-height: 60vh;
  background: #000;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 1rem;
}

.modal-img {
  max-width: 100%;
  max-height: 55vh;
  object-fit: contain;
  border-radius: 12px;
}

.modal-image-caption {
  padding: 1.5rem;
  text-align: center;
  background: rgba(10, 10, 12, 0.95);
}

.modal-image-caption h4 {
  color: var(--accent-cyan);
  font-size: 1.2rem;
  margin-bottom: 0.4rem;
}

.modal-image-caption p {
  color: var(--text-secondary);
  font-size: 0.95rem;
}
</style>