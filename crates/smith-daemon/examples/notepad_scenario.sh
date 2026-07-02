#!/usr/bin/env bash
# Пример сквозного сценария автоматизации Notepad через smithd.
#
# Запуск:
#   1. На Windows запустить: smithd --host 0.0.0.0 --port 8742
#   2. Из WSL выполнить: bash crates/smith-daemon/examples/notepad_scenario.sh

set -euo pipefail

# WSL2: Windows-хост доступен через IP шлюза по умолчанию.
# Если localhost forwarding включён, можно заменить на localhost.
HOST="${SMITHD_HOST:-$(ip route show | awk '/default/ {print $3}')}"
PORT="${SMITHD_PORT:-8742}"
BASE="http://${HOST}:${PORT}"

echo "Using smithd at ${BASE}"

# 1. Запуск Notepad
echo "Starting Notepad..."
START_RESP=$(curl -s -X POST "${BASE}/execute" \
  -H 'Content-Type: application/json' \
  -d '{"tool":"windows.process","config":{"action":"start","command":"notepad.exe"}}')
echo "start response: ${START_RESP}"

PID=$(echo "${START_RESP}" | python3 -c 'import sys, json; print(json.load(sys.stdin).get("result", {}).get("pid", ""))')
echo "Notepad PID: ${PID}"

# Даём Notepad время открыть окно
sleep 2

# 2. Поиск текстового поля (Edit-контрол) внутри процесса Notepad
#    Примечание: имя/AutomationId зависят от версии Windows/Notepad.
#    Для Windows 10 обычно подходит control_type="Edit" + pid.
#    Для Windows 11 может потребоваться name="Text editor" или class_name="RichEditD2DPT".
#    Если find не находит элемент, увеличьте sleep или измените селектор.
echo "Finding edit control..."
FIND_RESP=$(curl -s -X POST "${BASE}/execute" \
  -H 'Content-Type: application/json' \
  -d "{\"tool\":\"windows.find\",\"config\":{\"control_type\":\"Edit\",\"pid\":${PID},\"output_key\":\"notepad_editor\"}}")
echo "find response: ${FIND_RESP}"

# 3. Ввод текста в найденное поле
echo "Typing text..."
TYPE_RESP=$(curl -s -X POST "${BASE}/execute" \
  -H 'Content-Type: application/json' \
  -d '{"tool":"windows.input_text","config":{"element_key":"notepad_editor","text":"Hello from WSL via smithd!"}}')
echo "input_text response: ${TYPE_RESP}"

echo "Scenario complete."
