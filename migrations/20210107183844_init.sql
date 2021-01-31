CREATE OR REPLACE FUNCTION public.trigger_set_timestamp()
 RETURNS trigger
 LANGUAGE plpgsql
AS $function$
      BEGIN
        NEW."updatedAt" = NOW();
        RETURN NEW;
      END;
      $function$;


CREATE TYPE enum_assets_type AS ENUM (
	'image',
	'fav-icon',
	'page-og-image',
	'project-og-image');

CREATE TYPE enum_project_collaborators_accessType AS ENUM (
	'read',
	'write',
	'manage',
	'owner');

CREATE TABLE public.accounts (
	id uuid NOT NULL DEFAULT gen_random_uuid(),
	"authId" varchar(100) NOT NULL,
	"name" varchar(255) NULL,
	email varchar(100) NOT NULL,
	meta jsonb NULL DEFAULT '{}'::jsonb,
	picture text NULL,
	identities json NULL,
	"createdAt" timestamptz NOT NULL DEFAULT NOW(),
	"updatedAt" timestamptz NOT NULL DEFAULT NOW(),
	CONSTRAINT accounts_email_key UNIQUE (email),
	CONSTRAINT accounts_pkey PRIMARY KEY (id)
);
CREATE INDEX account_auth ON public.accounts USING btree ("authId");
CREATE INDEX account_email ON public.accounts USING btree (email);


create trigger update_account_updated_at before
update
    on
    public.accounts for each row execute function trigger_set_timestamp();


CREATE TABLE public.configs (
	id int NOT NULL,
	"predefinedProjects" _varchar NULL DEFAULT ARRAY[]::character varying[]::character varying(45)[],
	"almightyUsers" _varchar NULL DEFAULT ARRAY[]::character varying[]::character varying(30)[],
	"allowedEmails" _varchar NULL DEFAULT ARRAY[]::character varying[]::character varying(45)[],
	"allowedDomains" _varchar NULL DEFAULT ARRAY[]::character varying[]::character varying(30)[],
	CONSTRAINT configs_pkey PRIMARY KEY (id)
);

CREATE TABLE public.ops (
	collection varchar(255) NOT NULL,
	doc_id uuid NOT NULL,
	"version" int4 NOT NULL,
	operation jsonb NOT NULL,
	"createdAt" timestamptz NOT NULL DEFAULT NOW(),
	CONSTRAINT ops_pkey PRIMARY KEY (collection, doc_id, version)
);

create trigger update_ops_updated_at before
update
    on
    public.ops for each row execute function trigger_set_timestamp();


CREATE TABLE public.projects (
	id uuid NOT NULL DEFAULT gen_random_uuid(),
	"name" varchar(100) NOT NULL,
	slug varchar(100) NOT NULL,
	"createdAt" timestamptz NOT NULL DEFAULT NOW(),
	"updatedAt" timestamptz NOT NULL DEFAULT NOW(),
	CONSTRAINT projects_pkey PRIMARY KEY (id),
	CONSTRAINT projects_slug_key UNIQUE (slug)
);
CREATE INDEX project_slug ON public.projects USING btree (slug);

create trigger update_projects_updated_at before
update
    on
    public.projects for each row execute function trigger_set_timestamp();


CREATE TABLE public.sequelize_migrations (
	"name" varchar(255) NOT NULL,
	CONSTRAINT sequelize_migrations_pkey PRIMARY KEY (name)
);


CREATE TABLE public.assets (
	"path" varchar(100) NOT NULL,
	"projectId" uuid NOT NULL,
	mimetype varchar(50) NULL,
	"size" int4 NOT NULL,
	bucket varchar(40) NOT NULL,
	"type" enum_assets_type NOT NULL DEFAULT 'image'::enum_assets_type,
	"uploadedBy" uuid NOT NULL,
	"createdAt" timestamptz NOT NULL DEFAULT NOW(),
	"updatedAt" timestamptz NOT NULL DEFAULT NOW(),
	CONSTRAINT assets_pkey PRIMARY KEY (path, "projectId"),
	CONSTRAINT "assets_projectId_fkey" FOREIGN KEY ("projectId") REFERENCES projects(id) ON DELETE CASCADE,
	CONSTRAINT "assets_uploadedBy_fkey" FOREIGN KEY ("uploadedBy") REFERENCES accounts(id)
);
CREATE INDEX asset_uploader ON public.assets USING btree ("uploadedBy");


create trigger update_assets_updated_at before
update
    on
    public.assets for each row execute function trigger_set_timestamp();


CREATE TABLE public."project-collaborators" (
	id uuid NOT NULL DEFAULT gen_random_uuid(),
	"projectId" uuid NULL,
	"accountId" uuid NULL,
	"invitedEmail" varchar(100) NULL,
	meta jsonb NULL DEFAULT '{}'::jsonb,
	"accessType" enum_project_collaborators_accessType NOT NULL DEFAULT 'manage'::enum_project_collaborators_accessType,
	"createdAt" timestamptz NOT NULL DEFAULT NOW(),
	"updatedAt" timestamptz NOT NULL DEFAULT NOW(),
	CONSTRAINT "project-collaborators_pkey" PRIMARY KEY (id),
	CONSTRAINT "project-collaborators_accountId_fkey" FOREIGN KEY ("accountId") REFERENCES accounts(id) ON DELETE CASCADE,
	CONSTRAINT "project-collaborators_projectId_fkey" FOREIGN KEY ("projectId") REFERENCES projects(id) ON DELETE CASCADE
);
CREATE INDEX asset_project_id ON public."project-collaborators" USING btree ("projectId");
CREATE INDEX collaborator_account_id ON public."project-collaborators" USING btree ("accountId");
CREATE INDEX collaborator_invited_email ON public."project-collaborators" USING btree ("invitedEmail");
CREATE INDEX collaborator_project_id ON public."project-collaborators" USING btree ("projectId");


create trigger update_collaborators_updated_at before
update
    on
    public."project-collaborators" for each row execute function trigger_set_timestamp();


CREATE TABLE public.snapshots (
	collection varchar(255) NOT NULL,
	doc_id uuid NOT NULL,
	doc_type varchar(255) NOT NULL,
	"version" int4 NOT NULL,
	"data" jsonb NOT NULL,
	"updatedAt" timestamptz NOT NULL DEFAULT NOW(),
	CONSTRAINT snapshots_pkey PRIMARY KEY (collection, doc_id),
	CONSTRAINT snapshots_doc_id_fkey FOREIGN KEY (doc_id) REFERENCES projects(id) ON UPDATE CASCADE ON DELETE CASCADE
);

create trigger update_snapshots_updated_at before
update
    on
    public.snapshots for each row execute function trigger_set_timestamp();
