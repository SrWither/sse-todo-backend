DEFINE TABLE Todo SCHEMALESS;

DEFINE FIELD task ON TABLE Todo TYPE string
  ASSERT $value != NONE;
DEFINE FIELD completed ON TABLE Todo TYPE bool
  ASSERT $value != NONE;

CREATE Todo SET task = 'Prueba', completed: false
