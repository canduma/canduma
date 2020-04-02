// https://jestjs.io/docs/en/configuration#testenvironment-string

require('dotenv').config({ path: '.env' });
const knex = require('knex');
const NodeEnvironment = require('jest-environment-node');

class CustomEnvironment extends NodeEnvironment {
  constructor(config, context) {
    super(config, context);
    this.testPath = context.testPath;
    this.docblockPragmas = context.docblockPragmas;
  }

  async setup() {
    await super.setup();
    this.global.knex = knex({
      client: 'pg',
      connection: process.env.DATABASE_URL,
    });

    // Will trigger if docblock contains @my-custom-pragma my-pragma-value
    if (this.docblockPragmas['my-custom-pragma'] === 'my-pragma-value') {
      // ...
    }
  }

  async teardown() {
    this.global.knex.destroy();
    await super.teardown();
  }

  runScript(script) {
    return super.runScript(script);
  }

  // handleTestEvent(event, state) {
  //     if (event.name === 'test_start') {
  //         // ...
  //     }
  // }
}

module.exports = CustomEnvironment;
