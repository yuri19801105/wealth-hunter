#!/bin/bash
set -e

echo "=========================================="
echo "  Wealth Hunter 启动脚本"
echo "=========================================="
echo ""

# 检查环境变量
echo "🔍 检查环境变量..."

API_KEY="${GATEIO_API_KEY:-}"
API_SECRET="${GATEIO_API_SECRET:-}"
BASE_URL="${GATEIO_BASE_URL:-https://api.gateio.ws/api/v4}"

if [ -z "$API_KEY" ] || [ -z "$API_SECRET" ]; then
    echo "⚠️  警告: 未检测到 API Key，系统将运行在 Simulation 模式"
    echo "💡 提示: 请设置 GATEIO_API_KEY 和 GATEIO_API_SECRET 环境变量以启用 Live 模式"
    echo ""
    export TRADING_MODE="Simulation"
else
    echo "✅ 检测到 API Key，系统将运行在 Live 模式"
    export TRADING_MODE="Live"
fi

# 设置默认环境变量（如果未设置）
export RUST_LOG="${RUST_LOG:-warn}"
export API_PORT="${API_PORT:-8081}"

echo "📊 配置信息:"
echo "   - 交易模式: $TRADING_MODE"
echo "   - 日志级别: $RUST_LOG"
echo "   - API 端口: $API_PORT"
echo "   - API Base URL: $BASE_URL"
echo ""

# 检查数据目录
DATA_DIR="${DATA_DIR:-./data}"
LOG_DIR="${LOG_DIR:-./logs}"

if [ ! -d "$DATA_DIR" ]; then
    echo "📁 创建数据目录: $DATA_DIR"
    mkdir -p "$DATA_DIR"
fi

if [ ! -d "$LOG_DIR" ]; then
    echo "📁 创建日志目录: $LOG_DIR"
    mkdir -p "$LOG_DIR"
fi

# 检查数据库文件
DB_FILE="${DATA_DIR}/wealth_hunter.db"
if [ ! -f "$DB_FILE" ]; then
    echo "📄 数据库文件不存在，将创建新数据库: $DB_FILE"
fi

echo "=========================================="
echo "  启动 Wealth Hunter 系统"
echo "=========================================="
echo ""

# 启动 Wealth Hunter 后端
exec ./wealth_hunter
