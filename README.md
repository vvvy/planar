# planar

planar is a PoC of a modern serverless SQL data warehouse.

This work is currently in its very early stages.


## Source layout

| Dir             | Contents                                            |
|-----------------|-----------------------------------------------------|
| `planar-core`   | Core and common planar code                         |
| `sqlp`          | SQL Processor                                       |
| `cp`            | Compute Processor                                   |
| `ip`            | Ingest Processor                                    |
| `disp`          | Dispatcher                                          |
| `tmapi`         | Table Metadata API                                  |
| `pmapi`         | Process Metadata API                                |
| `pal`           | Cloud Provider Abstraction Layer library            |
| `p-local`       | Local runner for panar                              |
| `p-hc`          | Huawei Cloud support                                |