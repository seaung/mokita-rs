-- 创建自动更新 updated_at 的函数
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- 创建表结构
CREATE TABLE task_cron (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    task_uuid VARCHAR(36) NOT NULL,
    task_name VARCHAR(128) NOT NULL,
    task_type VARCHAR(32) NOT NULL,
    task_status VARCHAR(32) NOT NULL DEFAULT 'pending',
    task_cron_expr VARCHAR(128) NOT NULL,
    task_next_time TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uk_task_uuid UNIQUE (task_uuid)
);

-- 创建自动更新触发器
CREATE TRIGGER update_task_cron_modtime
    BEFORE UPDATE ON task_cron
    FOR EACH ROW
    EXECUTE FUNCTION update_modified_column();

-- 创建索引
CREATE INDEX idx_next_time ON task_cron(task_next_time);
CREATE INDEX idx_status ON task_cron(task_status);

-- 添加注释
COMMENT ON TABLE task_cron IS '定时任务配置表';
COMMENT ON COLUMN task_cron.id IS '主键ID';
COMMENT ON COLUMN task_cron.task_uuid IS '任务唯一标识';
COMMENT ON COLUMN task_cron.task_name IS '任务名称';
COMMENT ON COLUMN task_cron.task_type IS '任务类型';
COMMENT ON COLUMN task_cron.task_status IS '任务状态';
COMMENT ON COLUMN task_cron.task_cron_expr IS 'Cron表达式';
COMMENT ON COLUMN task_cron.task_next_time IS '下次执行时间';
COMMENT ON COLUMN task_cron.created_at IS '创建时间';
COMMENT ON COLUMN task_cron.updated_at IS '更新时间';
