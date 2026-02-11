-- USERS
INSERT INTO auth.users (username, password, enabled) VALUES
('john',  '$2a$10$qeS0HEh7urweMojsnwNAR.vcXJeXR1UcMRZ2WcGQl9YeuspUdgF.q', true),
('mary',  '$2a$10$qeS0HEh7urweMojsnwNAR.vcXJeXR1UcMRZ2WcGQl9YeuspUdgF.q', true),
('susan', '$2a$10$qeS0HEh7urweMojsnwNAR.vcXJeXR1UcMRZ2WcGQl9YeuspUdgF.q', true),
('admin', '$2a$10$qeS0HEh7urweMojsnwNAR.vcXJeXR1UcMRZ2WcGQl9YeuspUdgF.q', true);

-- ROLES
INSERT INTO auth.roles (name) VALUES
('ROLE_EMPLOYEE'),
('ROLE_MANAGER'),
('ROLE_ADMIN');

-- USER ↔ ROLE MAPPING
INSERT INTO auth.user_roles (user_id, role_id)
SELECT u.id, r.id
FROM auth.users u
JOIN auth.roles r
ON
  (u.username = 'john'  AND r.name = 'ROLE_EMPLOYEE')
  OR
  (u.username = 'mary'  AND r.name IN ('ROLE_EMPLOYEE','ROLE_MANAGER'))
  OR
  (u.username = 'susan' AND r.name IN ('ROLE_EMPLOYEE','ROLE_MANAGER','ROLE_ADMIN'))
  OR
  (u.username = 'admin' AND r.name IN ('ROLE_EMPLOYEE','ROLE_MANAGER','ROLE_ADMIN'));
