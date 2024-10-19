CREATE FUNCTION pfinance_refresh_all_views()
RETURNS void AS $$
BEGIN
    REFRESH MATERIALIZED VIEW budgets_overview;
END; $$
LANGUAGE plpgsql;
