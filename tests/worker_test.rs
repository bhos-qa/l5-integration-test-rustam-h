extern crate lab5;

use lab5::worker;

#[test]
fn test_get_repos() {
        assert_eq!(worker::get_repos(), r#"[{"id":"1","createdAt":"2021-05-16T13:57:15.119Z","name":"dolorum-quis-impedit"},{"id":"2","createdAt":"2021-05-16T21:45:33.803Z","name":"numquam-cupiditate-aut"},{"id":"3","createdAt":"2021-05-17T01:24:03.083Z","name":"dolorem-explicabo-aperiam"},{"id":"4","createdAt":"2021-05-17T01:03:43.831Z","name":"voluptas-consequuntur-nihil"},{"id":"5","createdAt":"2021-05-17T05:40:14.989Z","name":"omnis-ut-in"},{"id":"6","createdAt":"2021-05-16T19:16:30.942Z","name":"cumque-in-a"},{"id":"7","createdAt":"2021-05-17T03:01:18.297Z","name":"adipisci-accusantium-voluptatem"},{"id":"8","createdAt":"2021-05-17T02:52:51.646Z","name":"ex-provident-quos"},{"id":"9","createdAt":"2021-05-17T00:48:53.846Z","name":"provident-accusantium-commodi"},{"id":"10","createdAt":"2021-05-17T04:09:46.886Z","name":"modi-aliquam-maiores"},{"id":"11","createdAt":"2021-05-16T11:59:18.048Z","name":"molestiae-maxime-odio"},{"id":"12","createdAt":"2021-05-16T21:38:37.802Z","name":"laborum-reprehenderit-dolore"},{"id":"13","createdAt":"2021-05-16T15:36:28.121Z","name":"aut-ipsum-ut"},{"id":"14","createdAt":"2021-05-17T05:40:37.617Z","name":"aut-maiores-est"},{"id":"15","createdAt":"2021-05-17T04:07:37.366Z","name":"et-vero-qui"}]"#);
}

#[test]
fn test_get_branches() {
        assert_eq!(worker::get_branches(), r#"[{"id":"1","repoId":"1","createdAt":"2021-04-06T04:09:40.436Z","name":"circuit-parse","updatedAt":"2021-05-18T09:07:53.637Z"},{"id":"16","repoId":"1","createdAt":"2021-01-20T07:14:36.894Z","name":"application-override","updatedAt":"2021-05-18T13:22:38.996Z"},{"id":"31","repoId":"1","createdAt":"2020-11-29T07:03:51.310Z","name":"matrix-bypass","updatedAt":"2021-05-17T23:26:49.499Z"},{"id":"46","repoId":"1","createdAt":"2020-12-04T17:26:36.861Z","name":"bandwidth-calculate","updatedAt":"2021-05-18T04:07:25.281Z"}]"#);
}

#[test]
fn test_get_commits() {
        assert_eq!(worker::get_commits(), r#"[{"id":"1","branchId":"1","createdAt":"2021-05-18T11:56:32.045Z","message":"override multi-byte transmitter","entry":"commit d5fb726acb308b2306e375773d48a3f27a3a9b93\r\nMerge: 50c89cb ae6ee23\r\nAuthor: Kira Hessel <Anjali_Gulgowski@yahoo.com>\r\nDate: Tue May 18 2021 06:01:47 GMT+0000 (Coordinated Universal Time)\r\n\r\n    reboot primary bandwidth\r\n","sha":"183c83d459076b0821ae3ca73248e5c14e4e89d5"},{"id":"51","branchId":"1","createdAt":"2021-05-18T06:38:12.040Z","message":"parse back-end hard drive","entry":"commit 6c85c432fbfaec3743d01f2ab5b5cc9753074400\r\nAuthor: Einar Reichert <Moises13@gmail.com>\r\nDate: Tue May 18 2021 10:59:19 GMT+0000 (Coordinated Universal Time)\r\n\r\n    back up primary alarm\r\n","sha":"9140f60d1da615d4322e32d86475039c5ab14f40"}]"#);
}