# tired_server

This program detects if your server has been idle for a certain amount of time and shuts it down if it has.

## implementation

- systemstat is used to get the load average
https://github.com/valpackett/systemstat
- when the filter is triggered, the executor is called. You can run any bash scripts such as shutdown or reboot (sudo required), or send an API call using curl.


## Config example

```toml

[trigger]

record_interval = "60s"
grace_period = "10min"
threshold = "0.5"

[executor]

bash = "echo `system has been idle for 10 minutes`"


```

## sqlite query used

```sql
SELECT  MAX(load_5)
FROM    records
WHERE   update_ts > datetime('now', '-10 minutes')
```

```python

rows = conn.execute(query)
if len(rows) ==0:
    print("not enough data points")
    return False
else:
    return rows[0][0] < threshold

```