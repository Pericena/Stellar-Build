#!/usr/bin/env bash
# User tool — invocations signed by the investor / token holder.
# Replace placeholders before running on testnet.

set -euo pipefail

NETWORK="${NETWORK:-testnet}"
USER_KEY="${USER_KEY:-bob}"
CONTRACT_ID="${CONTRACT_ID:-C...DEPLOYED_LAUNCHPAD_CONTRACT_ID...}"
RECIPIENT="${RECIPIENT:-G...RECIPIENT_PUBLIC_KEY...}"

echo "=== invest 100 (must fail with AmountTooLow) ==="
set +e
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$USER_KEY" \
  --network "$NETWORK" \
  -- \
  invest \
  --investor "$(stellar keys address "$USER_KEY")" \
  --payment_amount 100
LOW_STATUS=$?
set -e
if [ "$LOW_STATUS" -eq 0 ]; then
  echo "ERROR: invest of 100 succeeded; AmountTooLow was expected" >&2
  exit 1
fi
echo "invest 100 failed as expected (exit ${LOW_STATUS})"

echo "=== invest 500 ==="
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$USER_KEY" \
  --network "$NETWORK" \
  -- \
  invest \
  --investor "$(stellar keys address "$USER_KEY")" \
  --payment_amount 500

echo "=== balance ==="
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$USER_KEY" \
  --network "$NETWORK" \
  -- \
  balance \
  --id "$(stellar keys address "$USER_KEY")"

if [ "${RUN_EXTRAS:-}" != "1" ]; then
  echo "=== user flow complete (invest 100 rejected, invest 500, balance) ==="
  echo "Set RUN_EXTRAS=1 to also transfer RWA tokens."
  exit 0
fi

echo "=== transfer RWA tokens ==="
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$USER_KEY" \
  --network "$NETWORK" \
  -- \
  transfer \
  --from "$(stellar keys address "$USER_KEY")" \
  --to "$RECIPIENT" \
  --amount 10
