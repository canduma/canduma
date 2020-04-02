const debug = require('debug')('canduma:user.test.js');
const request = require('supertest');

const HttpStatus = require('http-status-codes');

const apiPort = process.env.PORT || 3000;
const url = `http://localhost:${apiPort}`;

jest.setTimeout(500);

const name = 'my name';
const email = 'email1@nowhere.com';
const email2 = 'email2@nowhere.com';
const password = 'password';

async function cleanupDb() {
  return global.knex.raw('DELETE FROM users WHERE email in (?,?)', [
    email,
    email2,
  ]);
}
describe('user/', () => {
  beforeAll(async () => {
    return cleanupDb();
  });
  afterAll(async () => {
    return cleanupDb();
  });

  // const app = express();
  // app.use(cookieParser());
  //
  // app.get('/', function(req, res) {
  //     res.cookie('cookie', 'hey');
  //     res.send();
  // });
  //
  // app.get('/return', function(req, res) {
  //     if (req.cookies.cookie) res.send(req.cookies.cookie);
  //     else res.send(':(')
  // });

  const agent = request.agent(url);

  it('/user/me - UNAUTHORIZED before register', (done) => {
    agent
      .get('/user/me')
      .expect(HttpStatus.UNAUTHORIZED)
      .then(({ body, text }) => {
        debug('/user/me body=%o text=%s', body, text);
        expect(body).toBe('Unauthorized');
        expect(text).toBe('"Unauthorized"');
        done();
      });
  });

  it('/user/register - OK', (done) => {
    agent
      .post('/user/register')
      .send({ name, email, password })
      .expect(HttpStatus.OK)
      .then(({ body }) => {
        debug('/user/register body=%o', body);
        expect(body).toContainAllKeys(['user_uuid', 'email', 'role']);
        expect(body.email).toBe(email);
        expect(body.role).toBe('user');
        expect(body.user_uuid).not.toBeNull();
        done();
      });
  });

  it('/user/register - Bad Request', (done) => {
    agent
      .post('/user/register')
      .send({ name, email, password })
      .expect(HttpStatus.BAD_REQUEST)
      .then(({ body, error, text, headers }) => {
        debug(
          '/user/register body=%o text=%o error=%o headers=%o ',
          body,
          text,
          error,
          headers
        );
        expect(error.text).toBe(
          '\"Key (email)=(email1@nowhere.com) already exists.\"'
        );
        expect(body).toBe('Key (email)=(email1@nowhere.com) already exists.');
        done();
      });
  });

  it('/graphql:M register - OK', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            register( data: { name: "Test Name", email: "${email2}", password: "${password}" }) {
                email
                role
                userUuid
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql users=%o', body);
    const {
      data: { register },
    } = body;
    expect(register).toContainAllKeys(['email', 'role', 'userUuid']);
    expect(register.email).toBe(email2);
    expect(register.role).toBe('user');
    expect(register.userUuid).toBeNonEmptyString();
    done();
  });

  it('/graphql:M register - Key (email)=(email2@nowhere.com) already exists.', async (done) => {
    const { body } = await agent
      .post('/graphql')
      .send({
        query: `mutation  {
            register( data: { name: "Test Name", email: "${email2}", password: "${password}" }) {
                email
                role
                userUuid
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', body);
    const { errors, data } = body;
    expect(data).toBeNull();
    expect(errors[0].message).toBe(
      'Key (email)=(email2@nowhere.com) already exists.'
    );
    done();
  });

  it('/user/me - UNAUTHORIZED before login', (done) => {
    agent
      .get('/user/me')
      .expect(HttpStatus.UNAUTHORIZED)
      .then(({ body, text }) => {
        debug('/user/me body=%o text=%s', body, text);
        expect(text).toBe('"Unauthorized"');
        expect(body).toBe('Unauthorized');
        done();
      });
  });

  it('/user/login - UNAUTHORIZED with invalid email', (done) => {
    agent
      .post('/user/login')
      .send({ email: 'invalid@gmail.com', password })
      .expect(HttpStatus.UNAUTHORIZED)
      .then(({ body, text, error, headers }) => {
        debug(
          '/user/login body=%o text=%o error=%o headers=%o ',
          body,
          text,
          error,
          headers
        );
        expect(error.text).toBe('"Unauthorized"');
        expect(body).toBe('Unauthorized');
        done();
      });
  });

  it('/user/login - UNAUTHORIZED with invalid password', (done) => {
    agent
      .post('/user/login')
      .send({ email, password: 'invalid password' })
      .expect(HttpStatus.UNAUTHORIZED)
      .then(({ body, text, error, headers }) => {
        debug(
          '/user/login body=%o text=%o error=%o headers=%o ',
          body,
          text,
          error,
          headers
        );
        expect(error.text).toBe('"Unauthorized"');
        expect(body).toBe('Unauthorized');
        done();
      });
  });

  it('/user/login - OK to login first time', (done) => {
    agent
      .post('/user/login')
      .send({ email, password })
      .expect(HttpStatus.OK)
      .then(({ body, headers }) => {
        debug('/user/login headers=%o', headers);
        expect(headers['set-cookie'][0]).toBeNonEmptyString();
        expect(body).toContainAllKeys(['email', 'role', 'user_uuid']);
        expect(body.email).toBe(email);
        expect(body.role).toBe('user');
        expect(body.user_uuid).toBeNonEmptyString();
        done();
      });
  });

  it('/user/login - OK to login second time', (done) => {
    agent
      .post('/user/login')
      .send({ email, password })
      .expect(HttpStatus.OK)
      .then(({ body, headers }) => {
        expect(headers['set-cookie'][0]).toBeNonEmptyString();
        expect(body).toContainAllKeys(['email', 'role', 'user_uuid']);
        expect(body.email).toBe(email);
        expect(body.role).toBe('user');
        expect(body.user_uuid).toBeNonEmptyString();
        done();
      });
  });

  it('/graphql:Q users - OK', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query ListUsers {
            users {
                name
                userUuid
                email
                createdAt
            }
        }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql users=%o', response1.body.data.users);
    expect(response1.body.data.users).toBeNonEmptyArray();
    done();
  });

  it('/graphql:Q decodeToken - NO_ACCESS', async (done) => {
    const response3 = await agent
      .post('/graphql')
      .send({
        query: `query decodeTokenQuery {
          decodeToken {
              email
              iss
              iat
              exp
              sub
          }
      }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', response3.body);
    expect(response3.body.data).toBeNull();
    expect(response3.body.errors[0].message).toBe(
      'Unauthorized'
    );
    expect(response3.body.errors[0].path[0]).toBe('decodeToken');
    expect(response3.body.errors[0].extensions.type).toBe('NO_ACCESS');
    done();
  });

  it('/graphql:Q generateToken', async (done) => {
    const response1 = await agent
      .post('/graphql')
      .send({
        query: `query tokenQuery {
         generateToken {
            bearer
         }
       }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', response1.body);
    expect(response1.body.data.generateToken.bearer).toBeNonEmptyString();

    const response3 = await agent
      .post('/graphql')
      .set(
        'Authorization',
        `Bearer ${response1.body.data.generateToken.bearer}`
      )
      .send({
        query: `query decodeTokenQuery {
          decodeToken {
              email
              iss
              iat
              exp
              sub
          }
      }`,
      })
      .expect(HttpStatus.OK);
    debug('/graphql body=%o', response3.body);
    expect(response3.body.data.decodeToken.email).toBe(email);
    expect(response3.body.data.decodeToken.iss).toBe('localhost');

    done();
  });

  it('/user/me - OK', (done) => {
    agent
      .get('/user/me')
      .expect(HttpStatus.OK)
      .then(({ body, headers }) => {
        debug('/user/me body=%o', body);
        debug('/user/me headers=%o', headers);
        expect(body).toContainAllKeys(['email', 'role', 'user_uuid']);
        expect(body.email).toBe(email);
        expect(body.role).toBe('user');
        expect(body.user_uuid).toBeNonEmptyString();
        done();
      });
  });

  it('/user/logout - OK', (done) => {
    agent.get('/user/logout').expect(HttpStatus.OK, done);
  });

  it('/user/me - UNAUTHORIZED', (done) => {
    agent.get('/user/me').expect(HttpStatus.UNAUTHORIZED, done);
  });

  it('/graphql:Q users - You need to have role user, but have role bad_role', async (done) => {
    await global.knex.raw('UPDATE users SET role=? WHERE email=?', [
      'bad_role',
      email,
    ]);
    {
      const { body, headers } = await agent
        .post('/user/login')
        .send({ email, password })
        .expect(HttpStatus.OK);
      expect(headers['set-cookie'][0]).toBeNonEmptyString();
      expect(body).toContainAllKeys(['email', 'role', 'user_uuid']);
      expect(body.email).toBe(email);
      expect(body.role).toBe('bad_role');
      expect(body.user_uuid).toBeNonEmptyString();
    }
    {
      const response1 = await agent
        .post('/graphql')
        .send({
          query: `query ListUsers {
            users {
                name
                userUuid
                email
                createdAt
            }
        }`,
        })
        .expect(HttpStatus.OK);
      debug('/graphql body=%o', response1.body);
      expect(response1.body.errors[0].message).toBe(
        'Unauthorized'
      );
    }
    done();
  });
});
