# CONTEXT SUMMARY - PROYECTO ONLYCARNLD

## 🔬 RADIOGRAFÍA MOLECULAR GLOBAL DEL PROYECTO

> **Análisis forense algorítmico:** 2026-01-28 01:46
> **Herramientas:** `tree_viewer.py`, `dependency_mapper.py`, `project_stats.py`

### Esencia Nuclear del Proyecto

**OnlyCar es el "Servicio On-Demand premium accesible".** Plataforma abierta donde cualquiera puede ser operador y cualquiera puede solicitar servicio de limpieza vehicular. El proyecto integra 9 módulos documentales que cubren desde la identidad del negocio hasta el roadmap de escalamiento, con 833 archivos de documentación estructurados en jerarquías de hasta 6 niveles de profundidad.

**Los 6 Pilares:** Universal, Accesible, Democrático, Híbrido (B2B+B2C), On-demand, Friendly.

### Métricas Atómicas Globales

| Módulo | Archivos | % | Esencia |
|--------|----------|---|---------|
| 1.0 Identidad | 215 | 26% | El ALMA - visión, personas, comunicación |
| 2.0 Tecnología | 126 | 15% | El CUERPO - Nuxt+Capacitor+Supabase |
| 3.0 Datos | 194 | 23% | El CEREBRO - lógica de negocio, precios |
| 4.0 Necesidades | 98 | 12% | Los REQUISITOS - RF/RNF, API SSOT |
| 5.0 Integraciones | 88 | 11% | Las CONEXIONES - Stripe, Mifiel, Geo |
| 6.0 UI/UX | 55 | 7% | La PIEL - Design System, WCAG |
| 7.0 Arquitectura | 30 | 4% | La ESTRUCTURA - Hexagonal, TDD |
| 8.0 Legal | 20 | 2% | El MARCO LEGAL - LFPDPPP, ToS |
| 9.0 Planificación | 6 | 1% | El ROADMAP - fases, escalamiento |
| **TOTAL** | **833** | **100%** | **9 módulos** |

### Estados del Proyecto

| Estado | Cantidad | % |
|--------|----------|---|
| ✅ activo | 734 | 88.1% |
| ⏳ preparado | 40 | 4.8% |
| 🔮 futuro | 55 | 6.6% |
| ✓ aceptado | 3 | 0.4% |

### Conceptos Nucleares del Proyecto

1. **Modelo Híbrido B2B+B2C:** Plataforma abierta donde cualquiera puede ser operador
2. **Escala 43-73%:** 7 niveles de comisión por experiencia (Pirita→Diamante)
3. **Delegación Total:** Facturación, firmas y pagos delegados a PACs/PSCs certificados
4. **Invariantes:** Reglas de negocio inquebrantables definidas en 3.6.1
5. **Watchtower:** Algoritmo de seguridad que verifica operador en ubicación esperada

---

## VISION DEL PROYECTO
OnlyCar es el "Servicio On-Demand premium accesible". Plataforma abierta donde cualquiera puede ser operador y cualquiera puede solicitar servicio. Premium pero accesible. Servicio universal sin membresías obligatorias.

Los 6 Pilares: Universal, Accesible, Democratico, Hibrido (B2B+B2C), On-demand, Friendly.

---

## ESTADISTICAS GLOBALES
- Total archivos de documentacion: 833
- Modulos principales: 9
- Profundidad maxima: 6 niveles de jerarquia

---

## RESUMEN POR MODULO

### 1.0 IDENTIDAD Y CONTEXTO (215 archivos)
El ALMA del negocio. Visión, contratos B2B, perfiles de 6 tipos de usuario, chat estilo Messenger, calificación, **Estrategia de Crecimiento, Modelo Operativo y Marco Legal**.
Ver: [[Proyecto OnlyCarNLD/Datos/_summaries/_1.0_SUMMARY]]

### 2.0 TECNOLOGIA (126 archivos)
El CUERPO del sistema. Stack Nuxt 4 + Capacitor 6 + Supabase + Cloudflare. Arquitectura **Frontend (7), Mobile (6), Backend (5), Performance (2.17), Testing (2.18), Disaster Recovery (2.19)**.
Ver: [[Proyecto OnlyCarNLD/Datos/_summaries/_2.0_SUMMARY]]

### 3.0 DATOS Y LOGICA (194 archivos) ← MAS GRANDE
El CEREBRO QUÁNTICO. Lógica de precios, esquemas financieros (Costos, Pagos, Caja), **Simulación Financiera (3.5), Auditoría Lógica (3.6), Autosuficiencia (3.1.11.6) y Beneficios Laborales (3.1.12)**.
Ver: [[Proyecto OnlyCarNLD/Datos/_summaries/_3.0_SUMMARY]]

### 4.0 NECESIDADES DEL SISTEMA (98 archivos)
Los REQUISITOS. Funcionales y no funcionales, **API Specification (SSOT - Fuente Única de Verdad)**, ADRs, ambientes, trazabilidad.
Ver: [[Proyecto OnlyCarNLD/Datos/_summaries/_4.0_SUMMARY]]

### 5.0 INTEGRACIONES EXTERNAS (88 archivos)
Las CONEXIONES. Stripe, MercadoPago, Gigstack CFDI, Mifiel firmas, geolocalizacion (Watchtower), autenticacion OAuth, **Verificación Híbrida (Manual/Auto)**.
Ver: [[Proyecto OnlyCarNLD/Datos/_summaries/_5.0_SUMMARY]]

### 6.0 UI/UX (55 archivos)
La PIEL. Design system, componentes, pantallas por rol, accesibilidad WCAG, dark mode, animaciones.
Ver: [[Proyecto OnlyCarNLD/Datos/_summaries/_6.0_SUMMARY]]

### 7.0 ARQUITECTURA (30 archivos)
La ESTRUCTURA. Hexagonal Architecture, Clean Architecture, DDD, Atomic Design, Repository/Strategy Patterns, TDD, Git Flow, Docs-as-Code.
Ver: [[Proyecto OnlyCarNLD/Datos/_summaries/_7.0_SUMMARY]]

### 8.0 LEGAL Y CUMPLIMIENTO (20 archivos)
El MARCO LEGAL. Privacidad LFPDPPP 2025, términos de servicio, cumplimiento regulatorio (CFF, LFT), contratos con firma digital Mifiel, retención de datos.
Ver: [[Proyecto OnlyCarNLD/Datos/_summaries/_8.0_SUMMARY]]

### 9.0 PLANIFICACIÓN (6 archivos)
El ROADMAP. Planificación de entregas, gestión de riesgos, plan de escalamiento empresarial con hitos de transición y roadmap de contrataciones.
Ver: [[Proyecto OnlyCarNLD/Datos/_summaries/_9.0_SUMMARY]]

---

## MATRIZ DE DEPENDENCIAS CRITICAS

> IMPORTANTE: NO inventar dependencias que no esten en esta lista.

| Modulo Origen            | Depende De                     | Es Usado Por                  |
| ------------------------ | ------------------------------ | ----------------------------- |
| 1.2 operador             | 3.1 schemas, 5.8 geo           | 6.3 UI operador, 3.1.7 costos |
| 1.3 comunicacion         | 2.9 backend, Supabase Realtime | 6.4 UI chat, 1.4 calificacion |
| 3.1 data_JSON            | 2.0 tecnologia                 | 1.0, 4.0, 5.0 (todos)         |
| 3.1.7 costos             | 1.1 reglas, 3.1 schemas        | 5.1 pagos                     |
| 3.1.11.6 autosuficiencia | 3.1 precios, 3.5 simulacion    | 3.1.1 config, 3.1.7 costos    |
| 3.6 auditoria            | 3.1 schemas, 3.4 logs          | 5.1 pagos, 3.3 reglas         |
| 5.1 pagos                | 3.5 costos, 3.1 schemas        | 1.2 operador                  |
| 5.8 geo                  | 2.6 mobile                     | 1.2, 1.3, 6.0                 |
| 5.6 auth                 | Supabase, Google, Apple        | 1.2, 2.0                      |
| 6.0 UI                   | 2.5 frontend, 3.4 permisos     | Usuarios finales              |

**Flujo de Pagos:**
1.1.6 sistema_pagos → 5.1 stripe_pagos → 3.1.9 control_caja

**Flujo de Contratos B2B:**
1.1.7 contratos_b2b → 5.3 mifiel_firmas → 5.7 pdfme_generacion

**Flujo de Chat:**
1.3 comunicacion → 2.9.6 WebSocket_Architecture → 2.15 Arquitectura_Media

**Flujo de Ubicacion:**
5.8 geolocalizacion → 2.6 Arquitectura_Mobile → 1.3.6 chat_admin_operador

**Flujo de Operador:**
1.2.2 operador_perfil → 3.1.8 sistema_remuneracion → 3.1.9 control_caja

---

## GLOSARIO GLOBAL

| Termino         | Definicion                                                     |
| --------------- | -------------------------------------------------------------- |
| Operador        | Tecnico de servicio que ejecuta limpieza                       |
| B2C             | Cliente individual                                             |
| B2B             | Empresa con contrato                                           |
| Corporate+      | Empleado de empresa B2B con beneficios                         |
| RLS             | Row Level Security en PostgreSQL                               |
| Dynamic Pricing | Ajuste automático de precios por oferta/demanda                |
| Invariante      | Regla de lógica inquebrantable (Safety)                        |
| PAC             | Proveedor Autorizado de Certificacion CFDI                     |
| KYC             | Know Your Customer - verificacion identidad                    |
| Veriff          | Proveedor externo de verificacion facial                       |
| Reserva Legal   | Fondo obligatorio 5% utilidad hasta 20% capital (LGSM Art. 20) |
| Capitalización  | Aumento del capital social escriturado de la empresa           |
| GigWorker       | Operador independiente en modelo on-demand                     |
| Runway          | Meses de operación cubiertos por reservas líquidas             |
| ROE             | Return on Equity - Rentabilidad sobre capital                  |

---

## GUIA DE CHUNKING (Segmentacion de trabajo)

Regla: Dividir trabajo segun tamano del modulo para evitar saturacion IA.

| Tamano | Archivos | Estrategia |
|--------|----------|------------|
| Pequeno | <20 | 1 sesion completa |
| Mediano | 20-50 | 2 sesiones |
| Grande | 50-100 | 3-4 sesiones por submodulos |
| Muy Grande | >100 | Analizar submodulos individualmente |

**Aplicacion a OnlyCar:**

| Modulo | Archivos | Sesiones Recomendadas |
|--------|----------|----------------------|
| 1.0 Identidad | 215 | Por submodulos: 1.1, 1.2, 1.3, 1.4 |
| 3.0 Datos | 194 | Por submodulos: 3.1, 3.2, 3.3, 3.4, 3.5, 3.6 |
| 4.0 Requisitos | 98 | **(API SSOT)** Por submodulos |
| 5.0 Integraciones | 84 | Por submodulos |
| 2.0 Tecnologia | 81 | 2-3 sesiones |
| 6.0 UI/UX | 55 | 2-3 sesiones |
| 7.0 Arquitectura | 30 | 2 sesiones |
| 8.0 Legal y Cumplimiento | 20 | 1 sesion |
| 9.0 Planificacion | 6 | 1 sesion |

---

## ARCHIVOS MAS CRITICOS DEL PROYECTO

**Identidad:**
[[Proyecto OnlyCarNLD/Datos/1.1.0 vision_onlycar]] - Vision central
[[Proyecto OnlyCarNLD/Datos/1.1.7 contratos_b2b]] - Contratos empresariales
[[Proyecto OnlyCarNLD/Datos/1.2.2 operador_perfil]] - Flujo operador completo

**Tecnologia:**
[[Proyecto OnlyCarNLD/Datos/2.5. Arquitectura_Frontend]] - Base Nuxt
[[Proyecto OnlyCarNLD/Datos/2.19. Disaster_Recovery_Tech]] - Continuidad de Negocio

**Datos:**
[[Proyecto OnlyCarNLD/Datos/3.1.1 config_precios_v3.2]] - Configuracion precios
[[Proyecto OnlyCarNLD/Datos/3.1.8 sistema_remuneracion]] - Comisiones operador
[[Proyecto OnlyCarNLD/Datos/3.5.3 stress_test_financiero]] - Resiliencia Financiera
[[Proyecto OnlyCarNLD/Datos/3.6.1 invariantes_sistema]] - Reglas Inmutables

**Integraciones:**
[[Proyecto OnlyCarNLD/Datos/5.8. geolocalizacion]] - Tracking tiempo real (50 desc)
[[Proyecto OnlyCarNLD/Datos/5.6. autenticacion]] - Sistema auth dual

---

## PROTOCOLO DE ACTUALIZACION

Cuando se modifica documentacion de un modulo X.0:
1. Actualizar _X.0_SUMMARY.md con los cambios
2. Si cambian archivos criticos o dependencias, actualizar _CONTEXT_SUMMARY.md
3. Los SUMMARY no requieren verificacion de verify_project.py

---

**VERSION 3.7**
Actualizado: 2026-01-28
Basado en: 833 archivos de documentacion
Cambios v3.7: Incorporación de Radiografía Molecular Global con métricas atómicas, estados del proyecto y conceptos nucleares.
