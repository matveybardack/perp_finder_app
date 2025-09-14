# Feature Specification: Сервис агрегатор перп-маркетов криптобирж

**Feature Branch**: `001-drift-hyperliquid-kucoin`  
**Created**: 14.09.2025  
**Status**: Draft  
**Input**: User description: "Создать сервис для сбора, валидации и предоставления информации о перп-маркетах криптобирж (Drift, Hyperliquid, Kucoin, Gate, OKX, Binance), с хранением данных о всех доступных перп-парах, их ончейн-контрактах, объемах торгов, глубине стаканов, обновлением каждые 30 минут и внешним API для получения информации по контракту токена и сети. MVP — одна биржа, затем расширение. Источники для обогащения: CoinGecko, CoinMarketCap, дополнительные рекомендации. Возможность ручного обновления. Требуется архитектура, ТЗ и пошаговый план реализации."

## Execution Flow (main)
```
1. Parse user description from Input
2. Extract key concepts: биржи, перп-маркеты, токены, контракты, рыночные показатели, API, обновление данных
3. Ambiguities marked (см. ниже)
4. User Scenarios & Testing section filled
5. Functional Requirements generated
6. Key Entities identified (см. модель БД ниже)
7. Review Checklist
8. Return: SUCCESS (spec ready for planning)
```

---

## ⚡ Quick Guidelines
- ✅ Фокус на бизнес-ценности: агрегировать и предоставлять актуальные данные о перп-маркетах
- ❌ Не указывать детали реализации (кроме архитектуры MVP)
- 👥 Для бизнес-стейкхолдеров и разработчиков

---

## User Scenarios & Testing

### Primary User Story
Пользователь (разработчик, аналитик, трейдер) может получить через API актуальную информацию о перп-маркетах выбранной биржи, включая параметры рынка, контракты токенов, рыночные показатели и статус торговли.

### Acceptance Scenarios
1. **Given** биржа Kucoin активна, **When** пользователь запрашивает список перп-маркетов, **Then** сервис возвращает список с параметрами и статусами.
2. **Given** токен WLFI торгуется на Kucoin, **When** пользователь запрашивает контракты токена, **Then** сервис возвращает адреса контрактов в поддерживаемых сетях.
3. **Given** пользователь запрашивает рыночные показатели, **When** данные обновлены менее 30 минут назад, **Then** сервис возвращает актуальные значения.

### Edge Cases
- Если биржа недоступна:
	- Для MVP: приложение выводит сообщение об ошибке в консоль и завершает работу.
	- Для production: приложение пропускает недоступную биржу и продолжает обработку остальных.
- Как обрабатываются неоднозначные соответствия токенов? Собираются все варианты, для каждого ставится флаг ambiguous в БД.
- Какой формат ответа при отсутствии данных по токену?

---

## Requirements

### Functional Requirements
- **FR-001**: Сервис ДОЛЖЕН собирать и хранить данные о всех доступных перп-маркетах выбранной биржи (MVP: Kucoin)
- **FR-002**: Сервис ДОЛЖЕН валидировать и обогащать данные о токенах через внешние источники (CoinGecko, CoinMarketCap)
- **FR-003**: Сервис ДОЛЖЕН сохранять для каждого токена адреса контрактов в поддерживаемых сетях
- **FR-004**: Сервис ДОЛЖЕН хранить рыночные показатели: статус торговли, цена, 24h объём, min/max order, max leverage, funding rate
- **FR-005**: Сервис ДОЛЖЕН обновлять данные автоматически каждые 30 минут и по ручному запросу
- **FR-006**: Сервис ДОЛЖЕН предоставлять внешний API для получения информации по контракту токена, сети и рыночным показателям
- **FR-007**: Сервис ДОЛЖЕН хранить только актуальные данные (без истории)
- **FR-008**: Сервис ДОЛЖЕН обеспечивать время ответа API не более 300 мс, допускается отклонение до 500 мс.
- **FR-009**: Сервис ДОЛЖЕН поддерживать расширение на другие биржи без изменения архитектуры

- **FR-010**: Если обнаружено неоднозначное соответствие токенов, сервис ДОЛЖЕН помечать соответствующий токен или связку market-token через поле ambiguous BOOLEAN = true.

### Key Entities
- **Exchange**: Криптовалютная биржа (id, code, name, status, timestamps)
- **Market**: Перп-маркет (id, exchange_id, symbol_raw, base_symbol, quote_symbol, contract_type, is_tradable, price_step, qty_step, min/max order, max_leverage, funding_rate, timestamps)
- **Token**: Токен (id, slug, name, primary_symbol, ambiguous, timestamps)
- **MarketToken**: Связка market-token (market_id, token_id, role, ambiguous)
- **TokenContract**: Контракт токена (token_id, chain, address, decimals, source, verified, last_checked_at)
- **MarketSnapshot**: Снимок рынка (market_id, ts, last_price, volume_24h, funding_rate)
- **SlippageEstimate**: Оценка проскальзывания (market_id, ts, notional_usd, side, impact_price, slippage_abs, slippage_bps, method, depth_levels, meta)
- **AmbiguityQueue**: Очередь неоднозначностей (market_id, token_id, reason, created_at, resolved_at, resolution)

---

## Review & Acceptance Checklist
 [x] Нет деталей реализации (кроме архитектуры MVP)
 [x] Фокус на бизнес-ценности
 [x] Для не-технических и технических стейкхолдеров
 [x] Все обязательные секции заполнены
 [x] Нет [NEEDS CLARIFICATION] без ответа
 [x] Требования тестируемы и однозначны
 [x] Критерии успеха измеримы
 [x] Границы и зависимости определены

---

## Execution Status
- [ ] User description parsed
- [ ] Key concepts extracted
- [ ] Ambiguities marked
- [ ] User scenarios defined
- [ ] Requirements generated
- [ ] Entities identified
- [ ] Review checklist passed
