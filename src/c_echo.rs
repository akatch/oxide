use dicom_core::{ InMemDicomObject, StandardDataDictionary };
use dicom_core::data::{ DataElement, Tag };
use dicom_core::data::VR::*;
use dicom_core::data::value::{ Value, PrimitiveValue };

// build C-ECHO-RQ object
// http://dicom.nema.org/medical/dicom/current/output/chtml/part07/sect_9.3.5.html#table_9.3-12
pub fn build_rq(message_id: u16) -> InMemDicomObject<StandardDataDictionary> {
    let mut rq_object = InMemDicomObject::create_empty();

    // TODO constants for all available (supported?) command field values
    // could use these to build out DICOM conformance statement?
    let affected_sop_class_uid  = DataElement::new(Tag(0x0000, 0x0002), UI, Value::from(PrimitiveValue::Strs(vec![String::from("1.2.840.10008.1.1")])));
    let command_field           = DataElement::new(Tag(0x0000, 0x0100), US, Value::from(PrimitiveValue::U16(vec![0x0030])));
    let command_data_set_type   = DataElement::new(Tag(0x0000, 0x0800), US, Value::from(PrimitiveValue::U16(vec![0x0101])));

    let message_id              = DataElement::new(Tag(0x0000, 0x0110), US, Value::from(PrimitiveValue::U16(vec![message_id])));
    let command_group_length    = DataElement::new(Tag(0x0000, 0x0000), UL, Value::from(PrimitiveValue::U16(vec![0])));
    // TODO actually generate command_group_length
    //let command_group_length    = DataElement::new(Tag(0x0000, 0x0000), UL, rq_object.size().get().unwrap().to_string());

    let elements = vec![affected_sop_class_uid,
                        command_field,
                        message_id,
                        command_group_length,
                        command_data_set_type];

    // add required elements for C-ECHO-RQ
    for e in elements {
        InMemDicomObject::put(&mut rq_object, e);
    }

    rq_object
}

fn build_rsp() {
    // build C-ECHO-RSP
    // command group length         0x0000, 0x0000  UL  // size in bytes of this command group
    // affected service class UID   0x0000, 0x0002  UI  1.2.840.10008.1.1 // blue book table 7.2
    // command field                0x0000, 0x0100  US  0x8030
    // message ID responding to     0x0000, 0x0120  US  // value of 0000,0110 from RQ
    // data set type                0x0000, 0x0800  US  0x0101 // null data set
    // status                       0x0000, 0x0900  US  0x0000
}

#[test]
fn rq_is_valid() {
    let message_id = 0;
    let rq = build_rq(message_id);
    let sop_class_uid_element = rq.element(Tag(0x0000, 0x0002)).unwrap();
    let command_field_element = rq.element(Tag(0x0000, 0x0100)).unwrap();
    let message_id_element = rq.element(Tag(0x0000, 0x0110)).unwrap();
    let command_data_set_type_element = rq.element(Tag(0x0000, 0x0800)).unwrap();

    // check VRs
    assert_eq!(sop_class_uid_element.vr(), UI);
    assert_eq!(command_field_element.vr(), US);
    assert_eq!(message_id_element.vr(), US);
    assert_eq!(command_data_set_type_element.vr(), US);

    // check values
    assert_eq!(sop_class_uid_element.value().as_string().unwrap(), "1.2.840.10008.1.1");
    assert_eq!(command_field_element.value().primitive().unwrap(), &PrimitiveValue::U16(vec!(0x0030)));
    assert_eq!(command_data_set_type_element.value().primitive().unwrap(), &PrimitiveValue::U16(vec!(0x0101)));
}

#[test]
fn object_is_valid() {
    /*
     * what constitutes a valid dicom object?
     * - values must all be an even number of bytes in size
     * - values must use one of enum VR
     * - ensure file complies with PS3.5 7.4
     *   http://dicom.nema.org/medical/dicom/current/output/html/part05.html#chapter_7
     */
    assert!(false)
}
