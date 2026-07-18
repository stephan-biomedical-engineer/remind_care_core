const express = require('express');
const puppeteer = require('puppeteer');
const { Pool } = require('pg');
const jwt = require('jsonwebtoken');
const ejs = require('ejs');
const path = require('path');
const cors = require('cors');
require('dotenv').config();

const app = express();
const port = process.env.PORT || 3000;

app.use(express.json());
app.use(cors());

// Conexão com o banco de dados (mesmo do Rust)
const pool = new Pool({
  connectionString: process.env.DATABASE_URL,
});

// Middleware de Autenticação JWT (compatível com o do Rust)
const authenticate = (req, res, next) => {
  const authHeader = req.headers.authorization;
  if (!authHeader || !authHeader.startsWith('Bearer ')) {
    return res.status(401).json({ error: 'Token não fornecido ou inválido' });
  }

  const token = authHeader.split(' ')[1];
  try {
    const decoded = jwt.verify(token, process.env.JWT_SECRET);
    req.user = decoded; // { sub: user_id, exp, iat }
    next();
  } catch (err) {
    return res.status(401).json({ error: 'Token expirado ou inválido' });
  }
};

// Rota de Healthcheck
app.get('/health', (req, res) => {
  res.json({ status: 'ok', service: 'report_api' });
});

// Rota para Gerar Relatório em PDF
app.get('/doc', authenticate, async (req, res) => {
  const userId = req.user.sub;
  let browser = null;

  try {
    // 1. Buscar os dados do usuário e seus remédios
    const userResult = await pool.query('SELECT name, email FROM users WHERE id = $1', [userId]);
    if (userResult.rows.length === 0) {
      return res.status(404).json({ error: 'Usuário não encontrado' });
    }
    const user = userResult.rows[0];

    const medicinesResult = await pool.query(`
      SELECT m.id, m.name, m.dosage, m.compartment, m.scheduled_time, m.week_days, m.created_at,
             (SELECT COUNT(*) FROM medicine_logs l WHERE l.medicine_id = m.id AND l.situation = 'onTime') as on_time_count,
             (SELECT COUNT(*) FROM medicine_logs l WHERE l.medicine_id = m.id AND l.situation = 'early') as early_count,
             (SELECT COUNT(*) FROM medicine_logs l WHERE l.medicine_id = m.id AND l.situation IN ('late', 'missed')) as late_count,
             (SELECT COUNT(*) FROM medicine_logs l WHERE l.medicine_id = m.id AND l.situation = 'warning') as warning_count
      FROM medicines m 
      WHERE m.user_id = $1
      ORDER BY m.scheduled_time ASC
    `, [userId]);
    const medicines = medicinesResult.rows;

    // 2. Renderizar HTML com EJS
    const templatePath = path.join(__dirname, '../templates/report.ejs');
    const html = await ejs.renderFile(path.join(__dirname, '../templates/report.ejs'), {
      user: user,
      medicines: medicines,
      date: new Date().toLocaleString('pt-BR')
    });

    // 3. Gerar PDF com Puppeteer
    browser = await puppeteer.launch({
      headless: true,
      args: ['--no-sandbox', '--disable-setuid-sandbox', '--disable-dev-shm-usage'],
      executablePath: process.env.PUPPETEER_EXECUTABLE_PATH || undefined
    });
    const page = await browser.newPage();
    await page.setContent(html, { waitUntil: 'networkidle0' });
    
    const pdfBuffer = await page.pdf({
      format: 'A4',
      printBackground: true,
      margin: {
        top: '20px',
        bottom: '20px',
        left: '20px',
        right: '20px'
      }
    });

    // 4. Enviar PDF como resposta
    res.set({
      'Content-Type': 'application/pdf',
      'Content-Disposition': `attachment; filename="Relatorio_RemindCare_${new Date().getTime()}.pdf"`,
      'Content-Length': pdfBuffer.length
    });
    res.send(Buffer.from(pdfBuffer));

  } catch (error) {
    console.error('Erro ao gerar relatório:', error);
    res.status(500).json({ error: 'Falha interna ao gerar o relatório' });
  } finally {
    if (browser) {
      await browser.close();
    }
  }
});

// Rota para Retornar Dados de Estatísticas em JSON
app.get('/stats', authenticate, async (req, res) => {
  const userId = req.user.sub;
  try {
    const userResult = await pool.query('SELECT id, name, email FROM users WHERE id = $1', [userId]);
    if (userResult.rows.length === 0) {
      return res.status(404).json({ error: 'Usuário não encontrado' });
    }
    const user = userResult.rows[0];

    const medicinesResult = await pool.query(`
      SELECT m.id, m.name, m.dosage, m.compartment, m.scheduled_time, m.week_days, m.created_at,
             (SELECT COUNT(*) FROM medicine_logs l WHERE l.medicine_id = m.id AND l.situation = 'onTime') as on_time_count,
             (SELECT COUNT(*) FROM medicine_logs l WHERE l.medicine_id = m.id AND l.situation = 'early') as early_count,
             (SELECT COUNT(*) FROM medicine_logs l WHERE l.medicine_id = m.id AND l.situation IN ('late', 'missed')) as late_count,
             (SELECT COUNT(*) FROM medicine_logs l WHERE l.medicine_id = m.id AND l.situation = 'warning') as warning_count
      FROM medicines m 
      WHERE m.user_id = $1
      ORDER BY m.scheduled_time ASC
    `, [userId]);
    const medicines = medicinesResult.rows;

    const devicesResult = await pool.query(`
      SELECT id, firmware_version, created_at
      FROM devices
      WHERE user_id = $1
    `, [userId]);
    const devices = devicesResult.rows;

    res.json({
      user,
      devices,
      stats: medicines
    });
  } catch (error) {
    console.error('Erro ao buscar estatísticas:', error);
    res.status(500).json({ error: 'Falha interna ao buscar estatísticas' });
  }
});

app.listen(port, () => {
  console.log(`Report API rodando na porta ${port}`);
});
