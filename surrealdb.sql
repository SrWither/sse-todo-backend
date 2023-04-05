-- surreal start --log trace --user root --pass root file://$(pwd)/db

DEFINE TABLE Todo SCHEMAFULL;

DEFINE FIELD task ON TABLE Todo TYPE string
  ASSERT $value != NONE;
DEFINE FIELD completed ON TABLE Todo TYPE bool
  ASSERT $value != NONE;

CREATE Todo SET task = 'Test', completed = false;
