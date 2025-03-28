# BambangShop Receiver App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a Rocket web framework skeleton that you can work with.

As this is an Observer Design Pattern tutorial repository, you need to implement a feature: `Notification`.
This feature will receive notifications of creation, promotion, and deletion of a product, when this receiver instance is subscribed to a certain product type.
The notification will be sent using HTTP POST request, so you need to make the receiver endpoint in this project.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Receiver" folder.

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    ROCKET_PORT=8001
    APP_INSTANCE_ROOT_URL=http://localhost:${ROCKET_PORT}
    APP_PUBLISHER_ROOT_URL=http://localhost:8000
    APP_INSTANCE_NAME=Safira Sudrajat
    ```
    Here are the details of each environment variable:
    | variable                | type   | description                                                     |
    |-------------------------|--------|-----------------------------------------------------------------|
    | ROCKET_PORT             | string | Port number that will be listened by this receiver instance.    |
    | APP_INSTANCE_ROOT_URL   | string | URL address where this receiver instance can be accessed.       |
    | APP_PUUBLISHER_ROOT_URL | string | URL address where the publisher instance can be accessed.       |
    | APP_INSTANCE_NAME       | string | Name of this receiver instance, will be shown on notifications. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)
3.  To simulate multiple instances of BambangShop Receiver (as the tutorial mandates you to do so),
    you can open new terminal, then edit `ROCKET_PORT` in `.env` file, then execute another `cargo run`.

    For example, if you want to run 3 (three) instances of BambangShop Receiver at port `8001`, `8002`, and `8003`, you can do these steps:
    -   Edit `ROCKET_PORT` in `.env` to `8001`, then execute `cargo run`.
    -   Open new terminal, edit `ROCKET_PORT` in `.env` to `8002`, then execute `cargo run`.
    -   Open another new terminal, edit `ROCKET_PORT` in `.env` to `8003`, then execute `cargo run`.

## Mandatory Checklists (Subscriber)
-   [X] Clone https://gitlab.com/ichlaffterlalu/bambangshop-receiver to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [X] Commit: `Create Notification model struct.`
    -   [X] Commit: `Create SubscriberRequest model struct.`
    -   [X] Commit: `Create Notification database and Notification repository struct skeleton.`
    -   [X] Commit: `Implement add function in Notification repository.`
    -   [X] Commit: `Implement list_all_as_string function in Notification repository.`
    -   [X] Write answers of your learning module's "Reflection Subscriber-1" questions in this README.
-   **STAGE 3: Implement services and controllers**
    -   [X] Commit: `Create Notification service struct skeleton.`
    -   [X] Commit: `Implement subscribe function in Notification service.`
    -   [X] Commit: `Implement subscribe function in Notification controller.`
    -   [X] Commit: `Implement unsubscribe function in Notification service.`
    -   [X] Commit: `Implement unsubscribe function in Notification controller.`
    -   [X] Commit: `Implement receive_notification function in Notification service.`
    -   [X] Commit: `Implement receive function in Notification controller.`
    -   [X] Commit: `Implement list_messages function in Notification service.`
    -   [X] Commit: `Implement list function in Notification controller.`
    -   [X] Write answers of your learning module's "Reflection Subscriber-2" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Subscriber) Reflections

#### Reflection Subscriber-1

1. RwLock<> digunakan untuk menyinkronkan akses ke Vec dari Notification karena kita memerlukan akses bersamaan dari beberapa thread dimana operasi pembacaan lebih sering daripada penulisan. RwLock memungkinkan beberapa pembaca untuk mengakses data secara bersamaan tanpa saling menghalangi, sementara penulis harus menunggu sampai semua pembaca selesai. Ini berbeda dengan Mutex<> yang akan mengunci data untuk pembaca dan penulis, sehingga hanya satu thread yang dapat mengakses data pada satu waktu, yang kurang efisien jika operasi pembacaan lebih dominan.

2. Rust tidak mengizinkan mutasi variabel static secara langsung karena pendekatan keamanan memori yang ketat. Di Java, variabel static dapat diubah melalui fungsi static tetapi hal ini berpotensi menyebabkan kondisi balapan (race condition) karena tidak ada penguncian bawaan. Rust menerapkan sistem kepemilikan dan peminjaman yang ketat dimana mutasi variabel static dianggap tidak aman tanpa mekanisme sinkronisasi seperti RwLock atau Mutex. lazy_static digunakan untuk menginisialisasi data statis secara malas (saat pertama kali diakses) dan mengemas data dalam mekanisme sinkronisasi untuk memastikan akses yang aman dari berbagai thread.

#### Reflection Subscriber-2

1. Ya, saya telah mengeksplorasi bagian-bagian kode di luar tutorial, termasuk struktur `src/lib.rs` yang berisi konfigurasi dasar aplikasi seperti pengelolaan error, konfigurasi aplikasi, dan inisialisasi HTTP client dengan reqwest. Dari eksplorasi tersebut, saya memahami bahwa aplikasi ini menggunakan pola singleton untuk konfigurasi dengan lazy_static, menerapkan penanganan error yang tersentralisasi melalui fungsi `compose_error_response`, dan mengelola HTTP client sebagai resource yang dibagikan ke seluruh aplikasi. Struktur ini memudahkan pengembangan dan pemeliharaan aplikasi karena memisahkan konfigurasi dari logika bisnis dan membuat kode lebih terorganisir.

2. Pola Observer sangat mempermudah penambahan subscriber karena kita tidak perlu memodifikasi kode publisher untuk menambah subscriber baru. Ketika menjalankan multiple instance Receiver, setiap instance bisa mendaftar sebagai subscriber secara independen melalui endpoint `/subscribe/<product_type>` dan publisher akan secara otomatis mengirim notifikasi ke semua subscriber yang terdaftar. Untuk spawning multiple instance Main App (publisher), sistem ini tetap mudah dikelola karena setiap subscriber bisa mendaftar ke beberapa publisher yang berbeda tanpa perubahan kode, hanya perlu mengkonfigurasi URL yang tepat, meskipun perlu pengelolaan tambahan untuk memastikan konsistensi data antar publisher.

3. Saya telah membuat beberapa tes dan meningkatkan dokumentasi Postman collection untuk memastikan API berfungsi dengan benar. Penggunaan fitur-fitur ini sangat bermanfaat untuk pekerjaan saya, baik dalam tutorial maupun Group Project, karena membantu memverifikasi bahwa sistem berjalan sesuai harapan tanpa harus menguji secara manual. Dokumentasi pada Postman collection juga mempermudah anggota tim lain untuk memahami cara menggunakan API dan skenario penggunaan yang berbeda, sehingga meningkatkan produktivitas tim dan mengurangi kesalahpahaman dalam pengembangan.