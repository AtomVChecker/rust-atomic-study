# Atomic correlation analysis

Atomic correlations reflect AtomVChecker's ability to detect potential memory ordering misuses through static analysis(* means experimental result on atomic correlations from other applications in Table 3). All the experiments are done on an 20.04-Ubuntu system with a 3.20 GHz Intel processor.

| Library      | # Atomic Correlations | # Involved Structures | % False Positive  |
|--------------|-----------------------|-----------------------|-------------------|
| Once_Cell    | 11                    | 4                     | 9.09%             |
| Parking-lot  | 34                    | 6                     | 17.64%            |
| Rayon        | 21                    | 8                     | 14.29%            |
| Crossbeam    | 162                   | 26                    | 22.84%            |
| TiKV*        | 291                   | 42                    | 19.24%            |
| Teaclave*    | 163                   | 18                    | 13.12%            |
| Sled*        | 104                   | 20                    | 20.68%            |
| Servo*       | 42                    | 9                     | 11.31%            |
| InfluxDB*    | 67                    | 12                    | 18.44%            |
| Occlum*      | 51                    | 15                    | 14.12%            |
| SnarkOS*     | 22                    | 7                     | 12.95%            |
| Actix*       | 58                    | 14                    | 18.64%            |
| Tokio*       | 185                   | 37                    | 14.37%            |
| **Total**    | **1211**              | **218**               | **15.90%**        |
