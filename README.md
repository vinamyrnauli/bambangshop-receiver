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
-   [x] Clone https://gitlab.com/ichlaffterlalu/bambangshop-receiver to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create SubscriberRequest model struct.`
    -   [x] Commit: `Create Notification database and Notification repository struct skeleton.`
    -   [x] Commit: `Implement add function in Notification repository.`
    -   [x] Commit: `Implement list_all_as_string function in Notification repository.`
    -   [x] Write answers of your learning module's "Reflection Subscriber-1" questions in this README.
-   **STAGE 3: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Commit: `Implement receive_notification function in Notification service.`
    -   [x] Commit: `Implement receive function in Notification controller.`
    -   [x] Commit: `Implement list_messages function in Notification service.`
    -   [x] Commit: `Implement list function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Subscriber-2" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Subscriber) Reflections

#### Reflection Subscriber-1
###### 1. In this tutorial, we used **RwLock<>** to synchronise the use of **Vec** of **Notifications**. Explain why it is necessary for this case, and explain why we do not use **Mutex<>** instead?
* `RwLock<Vec>` sangat diperlukan untuk mengkoordinasikan penggunaan dengan struktur data, yaitu sebuah Vector yang berisi notifikasi di sepanjang *thread*. RwLock memungkinkan beberapa *reader* untuk mengakses data secara bersamaaan, sementara hanya ada satu *reader* yang bisa mengubahnya.
* Hal tersebut penting dalam *multi-threading* untuk menghindari persaingan data yang bisa terjadi secara acak dan memastikan bahwa informasi yang diakses selalu konsisten.
* Pilihan RwLock didasarkan pada strategi *locking* yang berbeda. RwLock memperbolehkan beberapa *reader* untuk *read lock* secara bersamaan untuk skenario apabila operasi *read* lebih sering tejadi daripada operasi *write*.
* Namun, Mutex hanya mengizinkan satu *thread* untuk memiliki akses eksklusif pada satu waktu yang bisa menghambat kinerja jika ada banyak operasi *read* yang terjadi secara bersamaan.

###### 2. In this tutorial, we used **lazy_static** external library to define **Vec** and **DashMap** as a **“static”** variable. Compared to Java where we can mutate the content of a **static** variable via a **static** function, why did not Rust allow us to do so?
* Pada Rust, tidak diizinkan untuk melakukan perubahan pada nilai *static variable* karena bertentangan dengan prinsip-prinsip *ownership* dan *borrowing*. Hal ini terkait dengan konsep *mutable aliasing*, di mana Rust sangat memperhatikan keamanan memori dan mencegah kesalahan data dengan menerapkan aturan terkait referensi *mutable* dan status bersama yang dapat dimutasi.
* Pada Java, diizinkan untuk mengubah nilai *static variable* melalui *static function* karena Java memiliki pendekatan yang lebih longgar dalam hal keselamatan memori. Java menggunakan metode sinkronisasi eksplisit, seperti *synchronized block* atau *volatile variable* untuk memastikan keamanan *thread*, berbeda dengan Rust yang lebih mengandalkan sistem *borrowing* dan *primitive concurrency*.
* Rust menempatkan prioritas utama pada keselamatan memori dengan menggunakan *borrowing* dan *primitive concurrency* seperti RwLock dan Mutex untuk memberikan *concurrency* yang aman dan efisien. Pendekatan ini membedakannya dari Java yang lebih sering menggunakan cara langsung untuk mengatur keamanan *thread*.

#### Reflection Subscriber-2
###### 1. Have you explored things outside of the steps in the tutorial, for example: **src/lib.rs**? If not, explain why you did not do so. If yes, explain things that you have learned from those other parts of code.
* Penggunaan *lazy_static* untuk inisialisasi *global variable* sangat bermanfaat bagi saya untuk memperdalam penggunaan *lazy_static* dalam Rust dengan menginisialisasi *global variable* secara efisien. *Global variable* diinisialisasi hanya saat pertama kali digunakan, mengurangi *overhead* saat program dimulai.
* Implementasi konfigurasi aplikasi dengan `.env` file membuat saya dapat memahami bahwa pengaturan seperti *port*, koneksi *database*, atau token API dapat disimpan secara terpusat dan mudah diakses.
* Penanganan kesalahan dengan tipe `Result<T,E>` bermanfaat bagi saya dalam mengelola kesalahan secara sistematis pada aplikasi Rust. Tipe *Result* memisahkan nilai sukses (Ok) dan nilai *error* (Err) untuk penanganan yang lebih terstruktur.
* Penggunaan `Custom<Json>` untuk respons *error* mengajarkan saya untuk meningkatkan kejelasan dan kegunaan informasi kesalahan yang disampaikan kepada *user*.

###### 2. Since you have completed the tutorial by now and have tried to test your notification system by spawning multiple instances of Receiver, explain how Observer pattern eases you to plug in more subscribers. How about spawning more than one instance of Main app, will it still be easy enough to add to the system?
* Penggunaan *Observer Pattern* dalam sistem notifikasi dapat menyederhanakan proses penambahan *subscriber* baru. Dengan menyimpan daftar *observer*, memungkinkan penambahan *subscriber* baru tanpa perlu mengubah struktur inti dari aplikasi. *Observer Pattern* memfasilitasi pengiriman notifikasi kepada semua *subscriber* yang terdaftar dengan cara yang lebih efisien.
* Saya juga mengalami tantangan saat menggunakan lebih dari satu *instance* di *main app*. Saat mencoba membuat beberapa *instance* dari aplikasi utama, setiap *instance* memiliki *Observer Pattern* sendiri. Jadi, jika ingin berbagi notifikasi antar *instance*, diperlukan mekanisme penyimpanan data bersama yang dapat diakses oleh semua *instance*. Hal tersebut dapat menimbulkan tantangan dalam memastikan sinkronisasi dan konsistensi antar-*instance* aplikasi, tertuama dalam pertukaran informasi atau notifikasi.

###### 3. Have you tried to make your own Tests, or enhance documentation on your **Postman collection**? If you have tried those features, tell us whether it is useful for your work (it can be your tutorial work or your Group Project).
* Walaupun saya belum membuat tes atau meningkatkan dokumentasi di Postman, saya yakin bahwa kedua fitur tersebut akan berguna untuk tugas kelompok saya nantinya.
* Dalam proyek kelompok, kemampuan untuk membuat tes yang komprehensif akan memastikan kinerja aplikasi kami. Selain itu, peningkatan dokumentasi di Postman akan memudahkan kolaborasi tim dalam pengembangan dan pengujian aplikasi. 