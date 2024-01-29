/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/datagram-transport-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/

pub trait DatagramWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<()>;
}

pub trait DatagramRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
}

pub trait DatagramTransport: DatagramWrite + DatagramRead {}

pub trait DatagramTransportMultiConnection: DatagramWrite {
    fn id(&self) -> u8;
}

pub trait DatagramTransportMulti {
    fn read(
        &mut self,
        buf: &mut [u8],
    ) -> std::io::Result<(impl DatagramTransportMultiConnection, usize)>;
}

#[cfg(test)]
mod tests {
    struct TestTransport {}

    impl DatagramWrite for TestTransport {
        fn write(&mut self, _data: &[u8]) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl DatagramRead for TestTransport {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            buf[0] = 0xca;
            Ok(1)
        }
    }

    struct TestMultiTransport {}
    struct TestMultiTransportConnection {
        id: u8,
    }

    impl DatagramWrite for TestMultiTransportConnection {
        fn write(&mut self, _data: &[u8]) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl DatagramTransportMultiConnection for TestMultiTransportConnection {
        fn id(&self) -> u8 {
            self.id
        }
    }

    impl DatagramTransportMulti for TestMultiTransport {
        fn read(
            &mut self,
            buf: &mut [u8],
        ) -> std::io::Result<(impl DatagramTransportMultiConnection, usize)> {
            buf[0] = 0xca;
            Ok((TestMultiTransportConnection { id: 1 }, 1))
        }
    }

    use super::*;

    #[test]
    fn single_write() {
        let mut transport = TestTransport {};
        let data: &[u8] = &[1, 2, 3, 4, 5];
        let result = transport.write(data);
        assert!(result.is_ok());
    }

    #[test]
    fn multi_read() {
        let mut transport = TestMultiTransport {};
        let mut buf = [0; 10];
        {
            let (mut connection, size) = transport.read(&mut buf).unwrap();
            assert!(size == 1);
            assert!(connection.id() == 1);
            let out_data: &[u8] = &[1, 2, 3, 4, 5];
            connection.write(&out_data).unwrap();
        }
        assert_eq!(buf[0], 0xca);
    }
}
