use opcua_types::{
    NodeId, QualifiedName, LocalizedText, AttributeId, DataValue, WriteMask, Variant,
    service_types::NodeClass,
    status_code::StatusCode,
};

use crate::{
    address_space::types::{Object, ObjectType, ReferenceType, Variable, VariableType, View, DataType, Method},
};

/// A `NodeType` is an enumeration holding every kind of node which can be hosted within the `AddressSpace`.
#[derive(Debug)]
pub enum NodeType {
    Object(Box<Object>),
    ObjectType(Box<ObjectType>),
    ReferenceType(Box<ReferenceType>),
    Variable(Box<Variable>),
    VariableType(Box<VariableType>),
    View(Box<View>),
    DataType(Box<DataType>),
    Method(Box<Method>),
}

pub trait HasNodeId {
    fn node_id(&self) -> NodeId;
}

impl HasNodeId for NodeType {
    fn node_id(&self) -> NodeId {
        self.as_node().node_id()
    }
}

impl NodeType {
    pub fn as_node(&self) -> &dyn Node {
        match *self {
            NodeType::Object(ref value) => value.as_ref(),
            NodeType::ObjectType(ref value) => value.as_ref(),
            NodeType::ReferenceType(ref value) => value.as_ref(),
            NodeType::Variable(ref value) => value.as_ref(),
            NodeType::VariableType(ref value) => value.as_ref(),
            NodeType::View(ref value) => value.as_ref(),
            NodeType::DataType(ref value) => value.as_ref(),
            NodeType::Method(ref value) => value.as_ref(),
        }
    }

    pub fn as_mut_node(&mut self) -> &mut dyn Node {
        match *self {
            NodeType::Object(ref mut value) => value.as_mut(),
            NodeType::ObjectType(ref mut value) => value.as_mut(),
            NodeType::ReferenceType(ref mut value) => value.as_mut(),
            NodeType::Variable(ref mut value) => value.as_mut(),
            NodeType::VariableType(ref mut value) => value.as_mut(),
            NodeType::View(ref mut value) => value.as_mut(),
            NodeType::DataType(ref mut value) => value.as_mut(),
            NodeType::Method(ref mut value) => value.as_mut(),
        }
    }

    // Returns the `NodeClass` of this `NodeType`.
    pub fn node_class(&self) -> NodeClass {
        match self {
            NodeType::Object(_) => NodeClass::Object,
            NodeType::ObjectType(_) => NodeClass::ObjectType,
            NodeType::ReferenceType(_) => NodeClass::ReferenceType,
            NodeType::Variable(_) => NodeClass::Variable,
            NodeType::VariableType(_) => NodeClass::VariableType,
            NodeType::View(_) => NodeClass::View,
            NodeType::DataType(_) => NodeClass::DataType,
            NodeType::Method(_) => NodeClass::Method,
        }
    }
}

/// Implemented within a macro for all Node types. Functions that return a result in an Option
/// do so because the attribute is optional and not necessarily there.
pub trait NodeBase {
    /// Returns the node class - Object, ObjectType, Method, DataType, ReferenceType, Variable, VariableType or View
    fn node_class(&self) -> NodeClass;

    /// Returns the node's `NodeId`
    fn node_id(&self) -> NodeId;

    /// Returns the node's browse name
    fn browse_name(&self) -> QualifiedName;

    /// Returns the node's display name
    fn display_name(&self) -> LocalizedText;

    /// Sets the node's display name
    fn set_display_name(&mut self, display_name: LocalizedText);

    fn description(&self) -> Option<LocalizedText>;

    fn set_description(&mut self, description: LocalizedText);

    fn write_mask(&self) -> Option<WriteMask>;

    fn set_write_mask(&mut self, write_mask: WriteMask);

    fn user_write_mask(&self) -> Option<WriteMask>;

    fn set_user_write_mask(&mut self, write_mask: WriteMask);
}

/// Implemented by each node type's to provide a generic way to set or get attributes, e.g.
/// from the Attributes service set. Internal callers could call the setter / getter on the node
/// if they have access to them.
pub trait Node: NodeBase {
    /// Finds the attribute and value. The param `max_age` is a hint in milliseconds:
    ///
    /// * value 0, server shall attempt to read a new value from the data source
    /// * value >= i32::max(), sever shall attempt to get a cached value
    ///
    /// If there is a getter registered with the node, then the getter will interpret
    /// `max_age` how it sees fit.
    fn get_attribute_max_age(&self, attribute_id: AttributeId, max_age: f64) -> Option<DataValue>;

    /// Finds the attribute and value.
    fn get_attribute(&self, attribute_id: AttributeId) -> Option<DataValue> {
        self.get_attribute_max_age(attribute_id, 0f64)
    }

    /// Sets the attribute with the new value
    fn set_attribute(&mut self, attribute_id: AttributeId, value: Variant) -> Result<(), StatusCode>;
}
