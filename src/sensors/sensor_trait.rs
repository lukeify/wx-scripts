use crate::database::WxDatabase;
use crate::sensors::Sensor;

pub trait SensorTrait {
    fn get_sensor(&self) -> &Sensor;
    fn get_args<T: >(&self) -> &T;
    fn monitor(&self);

    fn configure(&self) -> Result<(), rusqlite::Error> {
        let db = WxDatabase::new();
        db.insert_sensor(self.get_sensor())?;
        db.insert_sensor_arrangement(self.get_sensor(), self.get_args()).expect("TODO: panic message");
    }

    fn insert_data_entry(&self) {
        // WxDatabase::new().insert_sensor_arrangement()
        //     .expect("TODO: panic message");
    }

    /// TODO: Get the enum here instead of the string name of the
    fn sensor_val(&self) {
        match self {
            RainRadarSensor => println!("Foo"),
        };
    }
}
