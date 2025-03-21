Dalam proses membangun server web multithreaded di Rust, terdapat beberapa pembelajaran penting di setiap milestone. 

**Commit 1**
Pada Milestone 1, saya memahami bagaimana Rust menangani koneksi TCP menggunakan TcpListener dan TcpStream, serta cara membaca request dari client dengan BufReader. Namun, tantangan yang muncul adalah Rust memiliki borrow checking yang ketat, sehingga perlu berhati-hati dalam menangani referensi. Selain itu, penggunaan .unwrap() untuk error handling kurang aman, sehingga lebih baik menggunakan .expect() agar memberikan pesan error yang lebih jelas. Kelemahan utama dari implementasi awal ini adalah server hanya dapat menangani satu request dalam satu waktu, sehingga belum optimal untuk banyak pengguna.

**Commit 2**
![commit2](https://github.com/user-attachments/assets/13333f27-819d-4284-bf75-47dc998e36b7)

Pada Milestone 2, saya mulai mengimplementasikan multithreading dengan thread::spawn, yang memungkinkan server menangani beberapa request secara bersamaan. Namun, tanpa thread pool, membuat thread untuk setiap request bisa menyebabkan overhead yang besar dan penggunaan resource yang boros. Oleh karena itu, saya belajar bahwa penggunaan thread pool sangat penting untuk efisiensi, tetapi juga perlu diperhatikan agar tidak terjadi race condition atau deadlock. Rust membantu dalam mencegah kondisi balapan melalui sistem ownership dan borrowing, tetapi ini juga membuat implementasi lebih sulit dibandingkan dengan bahasa lain seperti Python atau JavaScript.

**Commit 3**
![commit3](https://github.com/user-attachments/assets/b653246f-e616-491e-be46-a7e8692521b4)
Selanjutnya, di Milestone 3, saya menguji performa server menggunakan alat benchmarking seperti wrk atau Apache Benchmark (ab). Hasil pengujian menunjukkan bahwa sebelum menggunakan multithreading, server mengalami bottleneck karena hanya dapat menangani satu permintaan dalam satu waktu. Namun, setelah menerapkan thread pool, performanya meningkat secara signifikan, meskipun tetap ada trade-off antara jumlah thread dan konsumsi resource. Dari milestone ini, saya belajar bahwa tools benchmarking sangat berguna dalam memahami batas kemampuan server dan menemukan potensi optimasi.

**Commit 4**
Di Milestone 4, saya mulai memisahkan logika dengan menggunakan struct agar lebih terorganisir. Dengan memisahkan thread pool dari fungsi utama server, kode menjadi lebih bersih dan lebih mudah dipelihara. Namun, Rust memiliki sistem ownership yang ketat, sehingga saya harus lebih memahami cara menangani kepemilikan data dalam struct. Saya juga mempelajari konsep Mutex dan Arc (Atomic Reference Counting) untuk mengelola state antar thread agar tidak terjadi race condition.

**Commit 5**
![image](https://github.com/user-attachments/assets/8ef271d8-ac89-48e0-9b8c-93b410c709cb)
Pada Milestone 5, saya mengganti metode new dengan build sebagai konvensi dalam pembuatan objek, sehingga kode lebih ekspresif dan mudah dipahami. Perubahan ini membantu meningkatkan readability dan maintainability, terutama dalam proyek berskala besar. Saya menyadari bahwa naming yang baik dalam metode sangat penting agar kode lebih mudah dimengerti oleh pengembang lain.

**Commit 6**
Akhirnya, pada Milestone 6, saya menyelesaikan server yang lebih stabil dan siap digunakan. Saya menambahkan error handling yang lebih baik untuk mencegah crash yang tidak terduga serta mengimplementasikan graceful shutdown, sehingga server dapat berhenti dengan baik saat program dihentikan. Dari keseluruhan proses ini, saya menyadari bahwa Rust memiliki kurva belajar yang cukup tinggi, tetapi memberikan keuntungan besar dalam hal keamanan memori dan performa tinggi.

Kesimpulannya, Rust menawarkan tantangan besar, tetapi hasilnya sangat memuaskan. Dari implementasi awal yang hanya dapat menangani satu request, kini server dapat menangani banyak request dengan lebih efisien melalui thread pool. Benchmarking membantu saya mengidentifikasi bottleneck dan mengoptimalkan performa, sedangkan refactoring dengan struct dan metode yang lebih baik membuat kode lebih bersih dan scalable. Ke depannya, saya tertarik untuk mengimplementasikan async Rust dengan tokio untuk performa yang lebih baik, serta menambahkan fitur logging dan monitoring untuk debugging yang lebih efektif.
