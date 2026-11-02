//! Large-key generation must produce the same rows for every partition plan.

use tpchgen::generators::{LineItemGenerator, OrderGenerator};

#[test]
fn large_scale_partitions_match_sequential_orders_and_lines() {
    // Cover the newly widened ranges while preserving dbgen's SF30000 RNG switch.
    for scale in [1.0, 10_738.0, 14_317.0, 20_000.0, 29_999.0] {
        let total_orders = OrderGenerator::calculate_row_count(scale, 1, 1);
        // A small early partition permits comparison with a short sequential prefix.
        let partitions = total_orders.min(2_000_000_000) as i32;
        let start_index = total_orders / partitions as i64;
        let expected_order = OrderGenerator::new(scale, 1, 1)
            .iter()
            .nth(start_index as usize)
            .unwrap();
        let actual_order = OrderGenerator::new(scale, 2, partitions)
            .iter()
            .next()
            .unwrap();
        assert_eq!(
            actual_order.to_string(),
            expected_order.to_string(),
            "orders at SF={scale}"
        );

        let order_key = actual_order.o_orderkey;
        let expected_lines: Vec<_> = LineItemGenerator::new(scale, 1, 1)
            .iter()
            .skip_while(|row| row.l_orderkey != order_key)
            .take_while(|row| row.l_orderkey == order_key)
            .map(|row| row.to_string())
            .collect();
        let actual_lines: Vec<_> = LineItemGenerator::new(scale, 2, partitions)
            .iter()
            .take_while(|row| row.l_orderkey == order_key)
            .map(|row| row.to_string())
            .collect();
        assert!(!expected_lines.is_empty());
        assert_eq!(actual_lines, expected_lines, "lineitem at SF={scale}");
    }
}
